use std::{convert::identity, usize};

use babylonia_terminal_sdk::{
    components::{
        dxvk_component::{self, DXVKComponent},
        proton_component::{self, ProtonComponent},
    },
    game_state::GameState,
    utils::github_requester::{GithubRelease, GithubRequester},
};
use log::{error, info};
use relm4::{
    self,
    gtk::{self, prelude::*},
    prelude::*,
    *,
};

use adw::prelude::*;
use libadwaita as adw;

use crate::{manager, ui::MainWindowMsg};

use super::SetupPageMsg;

#[derive(Debug)]
pub enum DownloadComponentsMsg {
    Next,
    UpdateGameState,
    UpdateProgressBar((u64, u64)), // current and max_progress
    UpdateProgressBarMsg(String),
    UpdateDownloadedComponentName(String),
}

#[derive(Debug)]
pub struct DownloadComponentsPage {
    //state
    game_state: GameState,

    // widgets
    proton_combo: adw::ComboRow,
    dxvk_combo: adw::ComboRow,

    // values
    proton_versions: Vec<GithubRelease>,
    dxvk_versions: Vec<GithubRelease>,
    selected_proton_version: Option<GithubRelease>,
    selected_dxvk_version: Option<GithubRelease>,

    //progress_bar
    progress_bar_reporter: std::sync::Arc<DownloadComponentProgressBarReporter>,
    progress_bar_message: String,
    fraction: f64,
    show_progress_bar: bool,

    // download part
    is_installing: bool,
    installation_handler: WorkerController<manager::HandleComponentInstallation>,
    downloaded_component_name: String,
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DownloadComponentsPage {
    type Input = DownloadComponentsMsg;

    type Output = SetupPageMsg;

    type Init = GameState;

    view! {
        #[root]
        gtk::Box {
            adw::PreferencesPage {
                set_hexpand: true,
                #[watch]
                set_visible: !model.is_installing,

                add = &adw::PreferencesGroup {
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    gtk::Label {
                        set_label: "Install components",
                        add_css_class: "title-1"
                    },
                },

                add = &adw::PreferencesGroup {
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    #[local_ref]
                    proton_combo -> adw::ComboRow {
                        set_title: "proton version",

                        set_model: Some(&gtk::StringList::new(model
                            .proton_versions
                            .iter()
                            .map(|r| r.tag_name.as_str())
                            .collect::<Vec<&str>>()
                            .as_slice())),
                    },

                    #[local_ref]
                    dxvk_combo -> adw::ComboRow {
                        set_title: "dxvk version",

                        set_model: Some(&gtk::StringList::new(model
                            .dxvk_versions
                            .iter()
                            .map(|r| r.tag_name.as_str())
                            .collect::<Vec<&str>>()
                            .as_slice())),
                    },
                },

                add = &adw::PreferencesGroup {
                    set_margin_vertical: 48,

                    gtk::Button {
                        set_css_classes: &["suggested-action", "pill"],

                        set_label: "Next",
                        set_hexpand: false,
                        set_width_request: 200,

                        connect_clicked => DownloadComponentsMsg::Next,
                    },
                },
            },

            adw::PreferencesPage {
                set_hexpand: true,
                #[watch]
                set_visible: model.is_installing,

                add = &adw::PreferencesGroup {
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    gtk::Label {
                        set_label: "Downloading and installing components",
                        add_css_class: "title-1"
                    },
                },

                add = &adw::PreferencesGroup {
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    adw::ActionRow {
                        #[watch]
                        set_title: match &model.selected_proton_version {
                            Some(release) => &release.tag_name,
                            None => "WTF??!! there's no proton version found ????",
                        },
                        set_subtitle: "Proton version",

                        #[watch]
                        set_icon_name: if model.game_state == GameState::ProtonNotInstalled { Some("emblem-ok-symbolic") } else { Some("process-working") },

                        add_prefix = &gtk::Spinner {
                            set_spinning: true,

                            #[watch]
                            set_visible: model.game_state == GameState::ProtonNotInstalled,
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    gtk::ProgressBar {
                        #[watch]
                        set_fraction: model.fraction,

                        #[watch]
                        set_text: Some(&model.progress_bar_message),
                        set_show_text: true,
                    }
                }
            },
        }
    }

    async fn init(
        game_state: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let proton_releases = ProtonComponent::get_github_releases(
            proton_component::PROTON_DEV,
            proton_component::PROTON_REPO,
        )
        .await
        .unwrap(); //TODO: remove unwrap()

        let dxvk_releases =
            DXVKComponent::get_github_releases(dxvk_component::DXVK_DEV, dxvk_component::DXVK_REPO)
                .await
                .unwrap(); //TODO: remove unwrap()

        let model = DownloadComponentsPage {
            game_state,

            proton_combo: adw::ComboRow::new(),
            dxvk_combo: adw::ComboRow::new(),

            proton_versions: proton_releases,
            dxvk_versions: dxvk_releases,
            selected_proton_version: None,
            selected_dxvk_version: None,

            progress_bar_reporter: DownloadComponentProgressBarReporter::create(sender.clone()),
            progress_bar_message: String::new(),
            fraction: 0.0,
            show_progress_bar: false,

            is_installing: false,
            installation_handler: manager::HandleComponentInstallation::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
            downloaded_component_name: String::new(),
        };

        let proton_combo = &model.proton_combo;
        let dxvk_combo = &model.dxvk_combo;

        let widgets = view_output!();

        AsyncComponentParts { widgets, model }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) -> () {
        match message {
            DownloadComponentsMsg::Next => {
                if !self.is_installing {
                    self.is_installing = true;

                    let proton_index = self.proton_combo.selected() as usize;
                    let dxvk_index = self.dxvk_combo.selected() as usize;

                    let proton_release = self.proton_versions[proton_index].clone();
                    let dxvk_release = self.dxvk_versions[dxvk_index].clone();

                    self.selected_proton_version = Some(proton_release);
                    self.selected_dxvk_version = Some(dxvk_release);
                    let _ = self.installation_handler.sender().send(
                        manager::HandleComponentInstallationMsg::StartInstallation((
                            proton_index,
                            dxvk_index,
                            self.progress_bar_reporter.clone(),
                        )),
                    );
                } else {
                    let _ = sender.output(SetupPageMsg::Finish);
                }
            }
            DownloadComponentsMsg::UpdateDownloadedComponentName(name) => {
                self.downloaded_component_name = name;
            }
            DownloadComponentsMsg::UpdateGameState => {
                self.game_state = GameState::get_current_state().await.unwrap();
            }
            DownloadComponentsMsg::UpdateProgressBar((current, max_progress)) => {
                self.fraction = if current == 0 {
                    0.0
                } else {
                    current as f64 / max_progress as f64
                };

                self.progress_bar_message = format!(
                    "Downloading {} : {:.2}%",
                    self.downloaded_component_name,
                    self.fraction * 100.0
                );
            }
            DownloadComponentsMsg::UpdateProgressBarMsg(message) => {
                self.progress_bar_message = message;
            }
        }
    }
}

#[derive(Debug)]
struct ProgressBarReporterPrivate {
    max_progress: Option<u64>,
}

#[derive(Debug)]
pub struct DownloadComponentProgressBarReporter {
    private: std::sync::Mutex<Option<ProgressBarReporterPrivate>>,
    sender: relm4::AsyncComponentSender<DownloadComponentsPage>,
}

impl DownloadComponentProgressBarReporter {
    fn create(page: relm4::AsyncComponentSender<DownloadComponentsPage>) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
            sender: page,
        })
    }
}

impl downloader::progress::Reporter for DownloadComponentProgressBarReporter {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let private = ProgressBarReporterPrivate { max_progress };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            self.sender.input(DownloadComponentsMsg::UpdateProgressBar((
                current,
                p.max_progress.unwrap(),
            )));
        }
    }

    fn set_message(&self, message: &str) {}

    fn done(&self) {
        self.sender.input(DownloadComponentsMsg::Next);
        let mut guard = self.private.lock().unwrap();
        *guard = None;
    }
}
