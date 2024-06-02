use std::thread;

use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_manager::GameManager, game_state::GameState,
};
use tokio_with_wasm::tokio;

use crate::messages::{
    error::ReportError,
    steps::dependencies::{NotifyDependenciesSuccessfullyInstalled, StartDependenciesInstallation},
};

pub async fn listen_dependecies_installation() {
    let mut receiver = StartDependenciesInstallation::get_dart_signal_receiver();
    while let Some(_) = receiver.recv().await {
        let proton_component = ProtonComponent::new(GameState::get_config().await.config_dir);
        let proton = proton_component.init_proton();
        if let Err(e) = proton {
            ReportError {
                error_message: format!("Failed to install DXVK : {}", e),
            }
            .send_signal_to_dart();
            continue;
        }

        thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    match GameManager::install_dependencies(&proton.unwrap()).await {
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
