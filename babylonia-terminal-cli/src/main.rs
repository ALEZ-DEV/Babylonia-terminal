use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_manager::GameManager, game_state::GameState,
};
use log::{debug, info, LevelFilter};
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
        .with_module_level("tokio_utils", LevelFilter::Off)
        .init()
        .unwrap();

    let mut proton_component: Option<ProtonComponent> = None;
    let mut proton: Option<Proton> = None;

    loop {
        let state = GameState::get_current_state().await;

        if GameState::get_current_state().await != GameState::WineNotInstalled
            && proton_component == None
        {
            let proton_component =
                ProtonComponent::new(GameState::get_config_directory().join("wine"));
            match proton_component.init_proton() {
                Ok(p) => proton = Some(p),
                Err(err) => panic!("{}", err),
            };
        }

        match state {
            GameState::WineNotInstalled => {
                info!("Wine not installed, installing it...");
                proton_component = Some(
                    GameManager::install_wine(
                        GameState::get_config_directory(),
                        Some(DownloadReporter::create(false)),
                    )
                    .await
                    .expect("Failed to install Wine"),
                );
                info!("Wine installed");
            }
            GameState::DXVKNotInstalled => {
                info!("DXVK not installed, installing it...");
                GameManager::install_dxvk(
                    &proton.clone().unwrap(),
                    GameState::get_config_directory(),
                    Some(DownloadReporter::create(false)),
                )
                .await
                .expect("Failed to installed DXVK");
                info!("DXVK installed");
            }
            GameState::FontNotInstalled => {
                info!("Fonts not installed, installing it...");
                GameManager::install_font(
                    &proton.clone().unwrap(),
                    DownloadReporter::create(false),
                )
                .await
                .expect("Failed to install fonts");
                info!("Fonts installed");
            }
            GameState::DependecieNotInstalled => {
                info!("Dependecies not installed, installing it...");
                GameManager::install_dependecies(&proton.clone().unwrap())
                    .await
                    .expect("Failed to install dependecies");
                info!("Dependecies installed");
            }
            GameState::GameNotInstalled => {
                info!("Game not installed, installing it...");
                GameManager::install_game(
                    GameState::get_config_directory().join("PGR"),
                    DownloadReporter::create(false),
                )
                .await
                .expect("Failed to install the game");
            }
            _ => {}
        }

        if GameState::get_current_state().await == GameState::GameInstalled {
            break;
        }
    }

    info!("Starting game...");
    debug!("{:?}", proton);
    GameManager::start_game(
        &proton.unwrap(),
        GameState::get_config_directory().join("PGR"),
    )
    .await;
}
