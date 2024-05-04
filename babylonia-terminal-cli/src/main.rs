use std::{path::PathBuf, str::FromStr};

use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_manager::GameManager, game_patcher,
    game_state::GameState,
};
use log::{debug, info, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use wincompatlib::prelude::*;

pub mod reporter;

use crate::reporter::DownloadReporter;

#[tokio::main]
async fn main() {
    let simple_logger = SimpleLogger::new()
        .with_module_level("hyper", LevelFilter::Off)
        .with_module_level("hyper_util", LevelFilter::Off)
        .with_module_level("tracing", LevelFilter::Off)
        .with_module_level("rustls", LevelFilter::Off)
        .with_module_level("minreq", LevelFilter::Off)
        .with_module_level("tokio_utils", LevelFilter::Off);

    if cfg!(debug_assertions) {
        simple_logger.init().unwrap();
    } else {
        simple_logger.with_level(LevelFilter::Info).init().unwrap();
    }

    let mut proton_component: Option<ProtonComponent> = None;
    let mut proton: Option<Proton> = None;

    loop {
        let state = GameState::get_current_state().await;

        if state != GameState::WineNotInstalled && proton == None {
            let proton_component = ProtonComponent::new(GameState::get_config_directory());
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
                debug!("{:?}", proton_component);
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
                GameManager::install_font(&proton.clone().unwrap())
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
                if GameState::get_game_dir().await.is_none() {
                    info!(
                        "You can choose where to put your game directory, (default '{}')",
                        GameState::get_config_directory().to_str().unwrap(),
                    );
                    info!("Please enter your wanted game directory : ");
                    let mut input = BufReader::new(tokio::io::stdin())
                        .lines()
                        .next_line()
                        .await
                        .unwrap();

                    let dir;
                    if let Some(i) = &mut input {
                        if i.is_empty() {
                            dir = GameState::get_config_directory();
                        } else {
                            dir = PathBuf::from_str(i).expect("This is not a valid directory!\n Please restart the launcher and put a valid path.");
                        }
                    } else {
                        dir = GameState::get_config_directory();
                    }

                    GameState::set_game_dir(Some(dir)).await.expect(
                        "Failed to save the game directory into the config file, please retry!",
                    );
                }

                GameManager::install_game(
                    GameState::get_game_dir().await.unwrap(),
                    DownloadReporter::create(false),
                )
                .await
                .expect("Failed to install the game");
            }
            GameState::GameNotPatched => {
                info!("Patching game...");
                GameManager::patch_game(GameState::get_game_dir().await.unwrap())
                    .await
                    .expect("Failed to patch the game");
                info!("Game patched!");
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
        GameState::get_game_dir()
            .await
            .expect("Failed to start game, the game directory was not found"),
    )
    .await;
}
