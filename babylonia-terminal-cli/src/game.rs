use std::{path::PathBuf, str::FromStr, sync::Arc};

use babylonia_terminal_sdk::{
    components::{
        dxvk_component::{DXVK_DEV, DXVK_REPO},
        proton_component::{ProtonComponent, PROTON_DEV, PROTON_REPO},
    },
    game_config::GameConfig,
    game_manager::{EnvironmentVariable, GameManager},
    game_state::GameState,
};

use log::{debug, info};
use tokio::io::{AsyncBufReadExt, BufReader};
use wincompatlib::prelude::*;

use crate::{reporter::DownloadReporter, utils};

pub async fn run(
    launch_options: Option<String>,
    env_vars: Vec<EnvironmentVariable>,
    show_logs: bool,
) {
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
            let proton_component = ProtonComponent::new(GameConfig::get_config_directory().await);
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
                        GameConfig::get_config_directory().await,
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
                    GameConfig::get_config_directory().await,
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
                if GameConfig::get_game_dir().await.is_none() {
                    info!(
                        "You can choose where to put your game directory, (default '{}')",
                        GameConfig::get_config_directory().await.to_str().unwrap(),
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
                            dir = GameConfig::get_config_directory().await;
                        } else {
                            dir = PathBuf::from_str(i).expect("This is not a valid directory!\n Please restart the launcher and put a valid path.");
                        }
                    } else {
                        dir = GameConfig::get_config_directory().await;
                    }

                    GameConfig::set_game_dir(Some(dir)).await.expect(
                        "Failed to save the game directory into the config file, please retry!",
                    );
                }

                GameManager::install_game(
                    GameConfig::get_game_dir().await.unwrap(),
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
                GameManager::patch_game(GameConfig::get_game_dir().await.unwrap())
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
        GameConfig::get_game_dir()
            .await
            .expect("Failed to start game, the game directory was not found"),
        launch_options,
        env_vars,
        show_logs,
    )
    .await;
}
