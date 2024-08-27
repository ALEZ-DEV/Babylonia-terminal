use babylonia_terminal_sdk::game_config::GameConfig;

use crate::messages::{
    config::{
        ConfigInput, ConfigOutput, GetLaunchOptionsInput, GetLaunchOptionsOutput,
        SetLaunchOptionsInput,
    },
    error::ReportError,
};

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

pub async fn listen_get_launch_options() {
    let mut receiver = GetLaunchOptionsInput::get_dart_signal_receiver().unwrap();
    while let Some(_) = receiver.recv().await {
        let launch_options_result = GameConfig::get_launch_options().await;
        if let Err(err) = launch_options_result {
            ReportError {
                error_message: format!("Failed to get launch options :\n{:?}", err),
            }
            .send_signal_to_dart();
            continue;
        }

        GetLaunchOptionsOutput {
            launch_options: launch_options_result.unwrap(),
        }
        .send_signal_to_dart();
    }
}

pub async fn listen_set_launch_options() {
    let mut receiver = SetLaunchOptionsInput::get_dart_signal_receiver().unwrap();
    while let Some(dart_signal) = receiver.recv().await {
        let result = GameConfig::set_launch_options(dart_signal.message.launch_options).await;
        if let Err(err) = result {
            ReportError {
                error_message: format!("Failed to set launch options :\n{:?}", err),
            }
            .send_signal_to_dart();
        }
    }
}
