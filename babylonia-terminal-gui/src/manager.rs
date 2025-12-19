use std::{ops::Deref, sync::Arc};

use babylonia_terminal_sdk::{
    components::wine_component::WineComponent, game_config::GameConfig, game_manager::GameManager,
    utils::github_requester::GithubRelease,
};
use downloader::download;
use log::{debug, error};
use relm4::{
    tokio::{self, sync::OnceCell},
    Worker,
};
use wincompatlib::prelude::Wine;

use crate::ui::{
    self,
    pages::{
        self,
        steps::{
            self,
            download_components::{self, DownloadComponentsPageWidgets},
        },
    },
};

static WINE: OnceCell<Wine> = OnceCell::const_new();

pub async fn get_wine() -> anyhow::Result<Wine> {
    if !WINE.initialized() {
        let wine_component = WineComponent::new(GameConfig::get_config().await.config_dir);
        let wine = wine_component.init_wine();

        if let Err(ref e) = wine {
            error!("Failed to initialize wine : {}", e);
            anyhow::bail!("Failed to initialize wine : {}", e);
        }

        Ok(WINE.get_or_init(|| async { wine.unwrap() }).await.clone())
    } else {
        Ok(WINE.get().unwrap().clone())
    }
}

pub async fn run_game() -> anyhow::Result<()> {
    let wine = get_wine().await?;
    let game_dir = GameConfig::get_config().await.game_dir;
    if game_dir.is_none() {
        error!("Failed to start game, the game directory was not found");
        anyhow::bail!("Failed to start game, the game directory was not found");
    }

    GameManager::start_game(&wine, game_dir.unwrap(), None, vec![], false).await?;

    Ok(())
}

#[derive(Debug)]
pub enum HandleGameProcessMsg {
    RunGame,
}

#[derive(Debug)]
pub struct HandleGameProcess;

impl Worker for HandleGameProcess {
    type Init = ();

    type Input = HandleGameProcessMsg;

    type Output = ui::pages::game::GamePageMsg;

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
                        sender.output(ui::pages::game::GamePageMsg::SetIsGameRunning(true));
                        if let Err(error) = run_game().await {
                            sender.output(pages::game::GamePageMsg::ShowError(format!(
                                "Unable to start game : {}",
                                error
                            )));
                        };
                        sender.output(ui::pages::game::GamePageMsg::SetIsGameRunning(false));
                    });
            }
        }
    }
}

#[derive(Debug)]
pub enum HandleGameInstallationMsg {
    StartInstallation(Arc<pages::game::ProgressBarGameInstallationReporter>),
    StartPatch,
    StartUpdate(Arc<pages::game::ProgressBarGameInstallationReporter>),
}

#[derive(Debug)]
pub struct HandleGameInstallation;

impl Worker for HandleGameInstallation {
    type Init = ();

    type Input = HandleGameInstallationMsg;

    type Output = pages::game::GamePageMsg;

    fn init(init: Self::Init, sender: relm4::ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            HandleGameInstallationMsg::StartInstallation(progress_bar) => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        sender.output(pages::game::GamePageMsg::SetIsDownloading(true));

                        let game_dir = if let Some(dir) = GameConfig::get_config().await.game_dir {
                            dir
                        } else {
                            GameConfig::get_config_directory().await
                        };

                        if let Err(error) = GameManager::install_game(game_dir, progress_bar).await
                        {
                            sender.output(pages::game::GamePageMsg::ShowError(format!(
                                "Error while downloading the game : {}",
                                error
                            )));
                        };

                        sender.output(pages::game::GamePageMsg::SetIsDownloading(false));
                        sender.output(pages::game::GamePageMsg::UpdateGameState);
                    });
            }
            HandleGameInstallationMsg::StartPatch => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        sender.output(pages::game::GamePageMsg::SetIsPatching(true));

                        let game_dir = if let Some(dir) = GameConfig::get_config().await.game_dir {
                            dir
                        } else {
                            GameConfig::get_config_directory().await
                        };

                        if let Err(error) = GameManager::patch_game(game_dir).await {
                            sender.output(pages::game::GamePageMsg::ShowError(format!(
                                "Error while patching the game : {}",
                                error
                            )));
                        };

                        sender.output(pages::game::GamePageMsg::SetIsPatching(false));
                        sender.output(pages::game::GamePageMsg::UpdateGameState);
                    });
            }
            HandleGameInstallationMsg::StartUpdate(progress_bar) => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        sender.output(pages::game::GamePageMsg::SetIsDownloading(true));

                        if let Err(error) = GameManager::update_game().await {
                            sender.output(pages::game::GamePageMsg::ShowError(format!(
                                "Error while updating the game : {}",
                                error
                            )));
                        };

                        let game_dir = if let Some(dir) = GameConfig::get_config().await.game_dir {
                            dir
                        } else {
                            GameConfig::get_config_directory().await
                        };

                        if let Err(error) = GameManager::install_game(game_dir, progress_bar).await
                        {
                            sender.output(pages::game::GamePageMsg::ShowError(format!(
                                "Error while updating the game : {}",
                                error
                            )));
                        };

                        sender.output(pages::game::GamePageMsg::SetIsDownloading(true));
                        sender.output(pages::game::GamePageMsg::UpdateGameState);
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
    ), // wine release and dxvk release
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
                wine_release,
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
                                String::from("Starting download for wine"),
                                Some(String::from("Unpacking and initializing wine")),
                            ),
                        );

                        let _ = sender.output(
                            download_components::DownloadComponentsMsg::UpdateCurrentlyInstalling(
                                download_components::CurrentlyInstalling::Wine,
                            ),
                        );

                        let _ = sender.output(
                            download_components::DownloadComponentsMsg::UpdateDownloadedComponentName(
                                String::from("wine"),
                            ),
                        );

                        let game_dir = if let Some(dir) = GameConfig::get_config().await.game_dir {
                            dir
                        } else {
                            GameConfig::get_config_directory().await
                        };

                        if let Err(error) = GameManager::install_wine(game_dir.clone(), wine_release, Some(progress_bar.clone())).await {
                            sender.output(download_components::DownloadComponentsMsg::ShowError(format!("Failed to install wine : {}", error))).unwrap();
                            return;
                        }

                        let _ = sender
                            .output(download_components::DownloadComponentsMsg::UpdateProgressBarMsg(String::from("Starting download for DXVK"), Some(String::from("Installing DXVK"))));

                        let _ = sender.output(download_components::DownloadComponentsMsg::UpdateCurrentlyInstalling(download_components::CurrentlyInstalling::DXVK));

                        let _ = sender.output(download_components::DownloadComponentsMsg::UpdateDownloadedComponentName(String::from("DXVK")));

                        let wine = match get_wine().await {
                            Ok(p) => p,
                            Err(e) => {
                                sender.output(download_components::DownloadComponentsMsg::ShowError(format!("Failed to initialize wine : {:?}", e))).unwrap();
                                return;
                            }
                        };

                        if let Err(error) = GameManager::install_dxvk(&wine, game_dir, dxvk_release, Some(progress_bar.clone())).await {
                            sender.output(download_components::DownloadComponentsMsg::ShowError(format!("Failed to install DXVK : {}", error))).unwrap();
                            return;
                        }

                        let _ = sender
                            .output(download_components::DownloadComponentsMsg::UpdateProgressBarMsg(String::from("Downloading and installing fonts"), Some(String::from("Fonts installed"))));

                        let _ = sender.output(download_components::DownloadComponentsMsg::UpdateCurrentlyInstalling(download_components::CurrentlyInstalling::Fonts));

                        let _ = sender.output(download_components::DownloadComponentsMsg::UpdateDownloadedComponentName(String::from("fonts")));

                        if let Err(error) = GameManager::install_font(&wine, Some(progress_bar.clone())).await {
                            sender.output(download_components::DownloadComponentsMsg::ShowError(format!("Failed to install fonts : {}", error))).unwrap();
                            return;
                        }

                        let _ = sender
                            .output(download_components::DownloadComponentsMsg::UpdateProgressBarMsg(String::from("Download and installing denpendecies"), None));

                        let _ = sender.output(download_components::DownloadComponentsMsg::UpdateCurrentlyInstalling(download_components::CurrentlyInstalling::Denpendecies));

                        let _ = sender.output(download_components::DownloadComponentsMsg::UpdateDownloadedComponentName(String::from("denpendecies")));

                        if let Err(error) = GameManager::install_dependencies(&wine).await {
                            sender.output(download_components::DownloadComponentsMsg::ShowError(format!("Failed to install dependencies : {}", error))).unwrap();
                            return;
                        }

                        debug!("Finished to installing the components!");

                        let _ = sender.output(download_components::DownloadComponentsMsg::Finish);
                    });
            }
        }
    }
}
