use std::{ops::Deref, sync::Arc};

use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_config::GameConfig,
    game_manager::GameManager, utils::github_requester::GithubRelease,
};
use log::error;
use relm4::{
    tokio::{self, sync::OnceCell},
    Worker,
};
use wincompatlib::prelude::Proton;

use crate::ui::{
    self,
    pages::steps::{self, download_components},
};

static PROTON: OnceCell<Proton> = OnceCell::const_new();

pub async fn get_proton() -> Proton {
    PROTON
        .get_or_init(|| async {
            let proton_component = ProtonComponent::new(GameConfig::get_config().await.config_dir);
            let proton = proton_component.init_proton();
            if let Err(ref e) = proton {
                error!("Failed to initialize proton : {}", e);
            }
            proton.unwrap()
        })
        .await
        .clone()
}

pub async fn run_game() {
    let proton = get_proton().await;
    let game_dir = GameConfig::get_game_dir().await;
    if game_dir.is_none() {
        error!("Failed to start game, the game directory was not found")
    }

    GameManager::start_game(&proton, game_dir.unwrap(), None, false).await;
}

#[derive(Debug)]
pub enum HandleGameProcessMsg {
    RunGame,
}

pub struct HandleGameProcess;

impl Worker for HandleGameProcess {
    type Init = ();

    type Input = HandleGameProcessMsg;

    type Output = ui::MainWindowMsg;

    fn init(_init: Self::Init, _sender: relm4::ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            HandleGameProcessMsg::RunGame => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        sender.output(ui::MainWindowMsg::SetIsGameRunning(true));
                        run_game().await;
                        sender.output(ui::MainWindowMsg::SetIsGameRunning(false));
                    });
            }
        }
    }
}

#[derive(Debug)]
pub enum HandleComponentInstallationMsg {
    StartInstallation(
        (
            usize,
            usize,
            Arc<download_components::DownloadComponentProgressBarReporter>,
        ),
    ), // proton release and dxvk release
}

#[derive(Debug)]
pub struct HandleComponentInstallation;

impl Worker for HandleComponentInstallation {
    type Init = ();

    type Input = HandleComponentInstallationMsg;

    type Output = download_components::DownloadComponentsMsg;

    fn init(_init: Self::Init, _sender: relm4::ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            HandleComponentInstallationMsg::StartInstallation((
                proton_release,
                dxvk_release,
                progress_bar,
            )) => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let _ = sender.output(
                            download_components::DownloadComponentsMsg::UpdateProgressBarMsg(
                                String::from("Starting download for proton"),
                            ),
                        );

                        let _ = sender.output(
                            download_components::DownloadComponentsMsg::UpdateDownloadedComponentName(
                                String::from("proton"),
                            ),
                        );

                        let game_dir = if let Some(dir) = GameConfig::get_config().await.game_dir {
                            dir
                        } else {
                            GameConfig::get_config_directory().await
                        };

                        GameManager::install_wine(game_dir, proton_release, Some(progress_bar))
                            .await;

                        let _ = sender.output(
                            download_components::DownloadComponentsMsg::UpdateProgressBarMsg(
                                String::from("Unpacking proton"),
                            ),
                        );

                        let _ = sender
                            .output(download_components::DownloadComponentsMsg::UpdateGameState);
                    });
            }
        }
    }
}
