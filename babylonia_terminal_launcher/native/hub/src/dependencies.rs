use std::thread;

use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_manager::GameManager, game_state::GameState,
};
use tokio_with_wasm::tokio;

use crate::{
    messages::{
        error::ReportError,
        steps::dependencies::{
            NotifyDependenciesSuccessfullyInstalled, StartDependenciesInstallation,
        },
    },
    proton::get_proton,
};

pub async fn listen_dependecies_installation() {
    let mut receiver = StartDependenciesInstallation::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let proton = get_proton().await;

        thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    match GameManager::install_dependencies(&proton).await {
                        Err(e) => ReportError {
                            error_message: format!("Failed to install dependencies : {}", e),
                        }
                        .send_signal_to_dart(),
                        Ok(_) => NotifyDependenciesSuccessfullyInstalled {}.send_signal_to_dart(),
                    }
                });
        });
    }
}
