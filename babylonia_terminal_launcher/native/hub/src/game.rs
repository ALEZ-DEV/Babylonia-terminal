use std::thread;

use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_manager::GameManager, game_state::GameState,
};
use rinf::debug_print;
use tokio_with_wasm::tokio;

use crate::messages::{
    error::ReportError,
    steps::game::{
        GameInstallationProgress, NotifyGameStartDownloading, NotifyGameStartPatching,
        NotifyGameSuccessfullyInstalled, StartGameInstallation,
    },
};

pub async fn listen_game_installation() {
    let mut receiver = StartGameInstallation::get_dart_signal_receiver();
    while let Some(info) = receiver.recv().await {
        thread::spawn(move || {
            debug_print!("start downloading game...");
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    if info.message.is_updating {
                        if let Err(e) = GameManager::update_game().await {
                            ReportError {
                                error_message: format!("Failed to start updating the game : {}", e),
                            }
                            .send_signal_to_dart();
                        }
                    }

                    let game_dir = GameState::get_config().await.config_dir;

                    match GameManager::install_game(
                        game_dir.clone(),
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
                            match GameManager::patch_game(game_dir).await {
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
