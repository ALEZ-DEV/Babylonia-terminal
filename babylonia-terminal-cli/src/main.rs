use std::{path::PathBuf, str::FromStr, sync::Arc};

use babylonia_terminal_sdk::{
    components::{
        dxvk_component::{DXVK_DEV, DXVK_REPO},
        proton_component::{ProtonComponent, PROTON_DEV, PROTON_REPO},
    },
    game_manager::GameManager,
    game_state::GameState,
};
use clap::Parser;
use log::{debug, info, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::io::{AsyncBufReadExt, BufReader};
use wincompatlib::prelude::*;

mod arguments;
pub mod reporter;
pub mod utils;

use crate::arguments::Args;
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

    let args = Args::parse();
    debug!("Launch option -> {:?}", args.options);

    let mut proton_component: Option<ProtonComponent> = None;
    let mut proton: Option<Proton> = None;

    loop {
        let state_result = GameState::get_current_state().await;
        if let Err(error) = state_result {
            info!("Something goes wrong : {:?}", error);
            break;
        }
        let state = state_result.unwrap();

        if state != GameState::ProtonNotInstalled && proton == None {
            let proton_component = ProtonComponent::new(GameState::get_config_directory().await);
            match proton_component.init_proton() {
                Ok(p) => proton = Some(p),
                Err(err) => panic!("{}", err),
            };
        }

        match state {
            GameState::ProtonNotInstalled => {
                let release;
                if utils::use_latest("Do you want to install latest version of Proton GE or a specific version of it?") {
                    release = 0;
                } else {
                    release = utils::choose_release_version(
                        PROTON_DEV,
                        PROTON_REPO,
                        "Please, select a version of Proton GE to install.",
                    )
                    .await
                    .expect("Failed to fetch proton version!");
                }

                info!("Proton not installed, installing it...");
                proton_component = Some(
                    GameManager::install_wine(
                        GameState::get_config_directory().await,
                        release,
                        Some(DownloadReporter::create(false)),
                    )
                    .await
                    .expect("Failed to install Wine"),
                );
                info!("Proton installed");
            }
            GameState::DXVKNotInstalled => {
                let release;
                if utils::use_latest(
                    "Do you want to install latest version of DXVK or a specific version of it?",
                ) {
                    release = 0;
                } else {
                    release = utils::choose_release_version(
                        DXVK_DEV,
                        DXVK_REPO,
                        "Please, select a version of DXVK to install.",
                    )
                    .await
                    .expect("Failed to fetch DXVK version!");
                }

                info!("DXVK not installed, installing it...");
                debug!("{:?}", proton_component);
                GameManager::install_dxvk(
                    &proton.clone().unwrap(),
                    GameState::get_config_directory().await,
                    release,
                    Some(DownloadReporter::create(false)),
                )
                .await
                .expect("Failed to installed DXVK");
                info!("DXVK installed");
            }
            GameState::FontNotInstalled => {
                info!("Fonts not installed, installing it...");
                GameManager::install_font(&proton.clone().unwrap(), None::<Arc<DownloadReporter>>)
                    .await
                    .expect("Failed to install fonts");
                info!("Fonts installed");
            }
            GameState::DependecieNotInstalled => {
                info!("Dependecies not installed, installing it...");
                GameManager::install_dependencies(&proton.clone().unwrap())
                    .await
                    .expect("Failed to install dependecies");
                info!("Dependecies installed");
            }
            GameState::GameNotInstalled => {
                info!("Game not installed, installing it...");
                if GameState::get_game_dir().await.is_none() {
                    info!(
                        "You can choose where to put your game directory, (default '{}')",
                        GameState::get_config_directory().await.to_str().unwrap(),
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
                            dir = GameState::get_config_directory().await;
                        } else {
                            dir = PathBuf::from_str(i).expect("This is not a valid directory!\n Please restart the launcher and put a valid path.");
                        }
                    } else {
                        dir = GameState::get_config_directory().await;
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
            GameState::GameNeedUpdate => {
                info!("Game need an update, updating it");
                info!("This will restart the installation process...");
                GameManager::update_game()
                    .await
                    .expect("Failed to start the installation process");
            }
            GameState::GameNotPatched => {
                info!("Patching game...");
                GameManager::patch_game(GameState::get_game_dir().await.unwrap())
                    .await
                    .expect("Failed to patch the game");
                info!("Game patched!");
            }
            GameState::GameInstalled => {
                break;
            }
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
