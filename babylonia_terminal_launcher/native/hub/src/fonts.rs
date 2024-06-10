use std::thread;

use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_manager::GameManager, game_state::GameState,
};
use tokio_with_wasm::tokio;

use crate::{
    messages::{
        error::ReportError,
        steps::fonts::{
            FontsInstallationProgress, NotifyFontsSuccessfullyInstalled, StartFontsInstallation,
        },
    },
    proton::get_proton,
};

pub async fn listen_fonts_installation() {
    let mut receiver = StartFontsInstallation::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let proton = get_proton().await;

        thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    match GameManager::install_font(&proton, Some(InstallationReporter::create()))
                        .await
                    {
                        Err(e) => ReportError {
                            error_message: format!("Failed to install DXVK : {}", e),
                        }
                        .send_signal_to_dart(),
                        Ok(_) => NotifyFontsSuccessfullyInstalled {}.send_signal_to_dart(),
                    }
                });
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
            FontsInstallationProgress {
                current,
                max: p.max_progress.unwrap(),
            }
            .send_signal_to_dart();
        }
    }

    fn set_message(&self, _: &str) {}

    fn done(&self) {
        let mut guard = self.private.lock().unwrap();
        *guard = None;
    }
}
