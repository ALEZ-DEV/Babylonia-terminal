use std::io;

use log::debug;
use tokio::fs::remove_dir_all;

use crate::game_config::GameConfig;

pub mod github_requester;
pub mod kuro_prod_api;

pub fn get_game_name() -> String {
    concat!("P", "G", "R").to_string()
}

pub fn get_game_name_with_executable() -> String {
    format!("{}.exe", get_game_name())
}

pub async fn remove_setup() -> io::Result<()> {
    let config_dir = GameConfig::get_config_directory().await;

    debug!("Current setup directory : {:?}", config_dir);

    remove_dir_all(config_dir).await
}
