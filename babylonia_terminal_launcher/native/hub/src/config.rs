use babylonia_terminal_sdk::game_config::GameConfig;

use crate::messages::config::{ConfigInput, ConfigOutput};

pub async fn listen_config() {
    let mut receiver = ConfigInput::get_dart_signal_receiver().unwrap();
    while let Some(_) = receiver.recv().await {
        let config = GameConfig::get_config().await;
        ConfigOutput {
            config_path: config.config_dir.to_str().unwrap().to_string(),
        }
        .send_signal_to_dart();
    }
}
