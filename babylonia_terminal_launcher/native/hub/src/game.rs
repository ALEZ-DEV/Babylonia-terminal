use std::thread;

use babylonia_terminal_sdk::{game_config::GameConfig, game_manager::GameManager};
use rinf::debug_print;
use tokio_with_wasm::tokio;

use crate::{
    messages::{
        error::ReportError,
        steps::game::{
            GameInstallationProgress, GameStopped, NotifyGameStartPatching,
            NotifyGameSuccessfullyInstalled, RunGame, StartGameInstallation,
        },
    },
    proton::get_proton,
};

pub async fn listen_game_running() {
    let mut receiver = RunGame::get_dart_signal_receiver().unwrap();
    thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                while let Some(_) = receiver.recv().await {
                    let proton = get_proton().await;
                    let game_dir = GameConfig::get_game_dir().await;
                    if game_dir.is_none() {
                        ReportError {
                            error_message: "Failed to start game, the game directory was not found"
                                .to_string(),
                        }
                        .send_signal_to_dart();
                        GameStopped {}.send_signal_to_dart();
                        continue;
                    }

                    GameManager::start_game(&proton, game_dir.unwrap(), None, false).await;

                    GameStopped {}.send_signal_to_dart();
                }
            })
    });
}

pub async fn listen_game_installation() {
    let mut receiver = StartGameInstallation::get_dart_signal_receiver().unwrap();
    while let Some(info) = receiver.recv().await {
        thread::spawn(move || {
            debug_print!("start downloading game...");
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    if GameConfig::get_game_dir().await.is_none() {
                        if let Err(e) =
                            GameConfig::set_game_dir(Some(GameConfig::get_config_directory().await))
                                .await
                        {
                            ReportError {
                                error_message: format!("Failed to set new path : {}", e),
                            }
                            .send_signal_to_dart();
                        }
                    }

                    if info.message.is_updating {
                        if let Err(e) = GameManager::update_game().await {
                            ReportError {
                                error_message: format!("Failed to start updating the game : {}", e),
                            }
                            .send_signal_to_dart();
                        }
                    }

                    let game_dir = GameConfig::get_game_dir().await;
                    if game_dir.is_none() {
                        ReportError {
                            error_message: "Failed to get the game directory".to_string(),
                        }
                        .send_signal_to_dart();
                    }

                    match GameManager::install_game(
                        game_dir.clone().unwrap(),
                        InstallationReporter::create(),
                    )
                    .await
                    {
                        Err(e) => ReportError {
                            error_message: format!("Failed to install game : {}", e),
                        }
                        .send_signal_to_dart(),
                        Ok(_) => {
                            NotifyGameStartPatching {}.send_signal_to_dart();
                            debug_print!("start patching game...");
                            match GameManager::patch_game(game_dir.unwrap()).await {
                                Err(e) => ReportError {
                                    error_message: format!("Failed to install game : {}", e),
                                }
                                .send_signal_to_dart(),
                                Ok(_) => NotifyGameSuccessfullyInstalled {}.send_signal_to_dart(),
                            }
                        }
                    }
                })
        });
    }
}

struct DownloadReporterPrivate {
    max_progress: Option<u64>,
}

struct InstallationReporter {
    private: std::sync::Mutex<Option<DownloadReporterPrivate>>,
}

impl InstallationReporter {
    pub fn create() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
        })
    }
}

impl downloader::progress::Reporter for InstallationReporter {
    fn setup(&self, max_progress: Option<u64>, _: &str) {
        let private = DownloadReporterPrivate { max_progress };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            GameInstallationProgress {
                current,
                max: p.max_progress.unwrap(),
            }
            .send_signal_to_dart();
        }
    }

    fn set_message(&self, _: &str) {}

    fn done(&self) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            GameInstallationProgress {
                current: p.max_progress.unwrap(),
                max: p.max_progress.unwrap(),
            }
            .send_signal_to_dart();
        }
        let mut guard = self.private.lock().unwrap();
        *guard = None;
    }
}
