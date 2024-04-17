use babylonia_terminal_sdk::{game_manager::GameManager, game_state::GameState};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use wincompatlib::prelude::*;

pub mod reporter;

use crate::reporter::DownloadReporter;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_module_level("hyper", LevelFilter::Off)
        .with_module_level("hyper_util", LevelFilter::Off)
        .with_module_level("tracing", LevelFilter::Off)
        .with_module_level("rustls", LevelFilter::Off)
        .with_module_level("minreq", LevelFilter::Off)
        .init()
        .unwrap();

    let mut wine: Option<Wine> = None;

    loop {
        let state = GameState::get_current_state().await;

        if state != GameState::WineNotInstalled && wine.is_none() {
            wine = Some(GameManager::init_wine());
        }

        match state {
            GameState::WineNotInstalled => {
                info!("Wine not installed, installing it...");
                GameManager::install_wine(
                    GameState::get_config_directory(),
                    Some(DownloadReporter::create()),
                )
                .await
                .expect("Failed to install Wine");
                info!("Wine installed");
            }
            GameState::DXVKNotInstalled => {
                info!("DXVK not installed, installing it...");
                GameManager::install_dxvk(
                    &wine.clone().unwrap(),
                    GameState::get_config_directory(),
                    Some(DownloadReporter::create()),
                )
                .await
                .expect("Failed to installed DXVK");
                info!("DXVK installed");
            }
            GameState::FontNotInstalled => {
                info!("Fonts not installed, installing it...");
                GameManager::install_font(&wine.clone().unwrap(), DownloadReporter::create())
                    .await
                    .expect("Failed to install fonts");
                info!("Fonts installed");
            }
            GameState::DependecieNotInstalled => {
                info!("Dependecies not installed, installing it...");
                GameManager::install_dependecies(&wine.clone().unwrap())
                    .await
                    .expect("Failed to install dependecies");
                info!("Dependecies installed");
            }
            _ => {}
        }

        if GameState::get_current_state().await == GameState::GameInstalled {
            break;
        }
    }

    info!("Starting game...");
    GameManager::start_game(&wine.unwrap()).await;
}
