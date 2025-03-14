use std::{convert::identity, usize};

use arboard::Clipboard;
use babylonia_terminal_sdk::{
    components::{
        dxvk_component::{self, DXVKComponent},
        proton_component::{self, ProtonComponent},
    },
    game_state::GameState,
    utils::{
        github_requester::{GithubRelease, GithubRequester},
        kuro_prod_api::CurrentGameInfo,
    },
};
use log::{debug, error, info};
use relm4::{
    self,
    gtk::{self, prelude::*},
    prelude::*,
    *,
};

use adw::prelude::*;
use libadwaita as adw;

use crate::{manager, ui::MAIN_WINDOW};

use super::SetupPageMsg;

#[derive(Debug)]
pub enum DownloadComponentsMsg {
    UpdateProgressBar((u64, u64)), // current and max_progress
    UpdateProgressBarMsg(String, Option<String>), // current msg and when done msg
    ShowDoneMsg,
    UpdateDownloadedComponentName(String),
    UpdateCurrentlyInstalling(CurrentlyInstalling),
    ShowError(String), // error message
    Finish,
    Quit,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentlyInstalling {
    None,
    Proton,
    DXVK,
    Fonts,
    Denpendecies,
}

#[derive(Debug)]
pub struct DownloadComponentsPage {
    // widgets
    proton_combo: adw::ComboRow,
    dxvk_combo: adw::ComboRow,
    //error_dialog: Controller<CopyDialog>,

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
    installation_handler: WorkerController<manager::HandleComponentInstallation>,
    downloaded_component_name: String,
    currently_installing: CurrentlyInstalling,
    msg_when_done: Option<String>,
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DownloadComponentsPage {
    type Input = DownloadComponentsMsg;

    type Output = SetupPageMsg;

    type Init = ();

    view! {
        #[root]
        gtk::Box {
            adw::PreferencesPage {
                set_hexpand: true,
                #[watch]
                set_visible: model.currently_installing == CurrentlyInstalling::None,

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

                        connect_clicked => DownloadComponentsMsg::UpdateCurrentlyInstalling(CurrentlyInstalling::Proton),
                    },
                },
            },

            adw::PreferencesPage {
                set_hexpand: true,
                #[watch]
                set_visible: model.currently_installing != CurrentlyInstalling::None,

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
                        set_title: "Proton",
                        #[watch]
                        set_subtitle: match &model.selected_proton_version {
                            Some(release) => &release.tag_name,
                            None => "WTF??!! there's no proton version found ????",
                        },

                        #[watch]
                        set_icon_name: if model.currently_installing == CurrentlyInstalling::Proton { Some("process-working") } else { Some("emblem-ok-symbolic") },

                        add_prefix = &gtk::Spinner {
                            set_spinning: true,

                            #[watch]
                            set_visible: model.currently_installing == CurrentlyInstalling::Proton,
                        }
                    },

                    adw::ActionRow {
                        set_title: "DXVK",
                        #[watch]
                        set_subtitle: match &model.selected_dxvk_version {
                            Some(release) => &release.tag_name,
                            None => "WTF??!! there's no proton version found ????",
                        },

                        #[watch]
                        set_icon_name: if model.currently_installing == CurrentlyInstalling::DXVK { Some("process-working") } else { Some("emblem-ok-symbolic") },

                        add_prefix = &gtk::Spinner {
                            set_spinning: true,

                            #[watch]
                            set_visible: model.currently_installing == CurrentlyInstalling::DXVK,
                        }
                    },

                    adw::ActionRow {
                        #[watch]
                        set_title: "Fonts",
                        set_subtitle: "Arial",

                        #[watch]
                        set_icon_name: if model.currently_installing == CurrentlyInstalling::Fonts { Some("process-working") } else { Some("emblem-ok-symbolic") },

                        add_prefix = &gtk::Spinner {
                            set_spinning: true,

                            #[watch]
                            set_visible: model.currently_installing == CurrentlyInstalling::Fonts,
                        }
                    },

                    adw::ActionRow {
                        #[watch]
                        set_title: "vcrun2022",
                        set_subtitle: "Denpendecies",

                        #[watch]
                        set_icon_name: if model.currently_installing == CurrentlyInstalling::Denpendecies { Some("process-working") } else { Some("emblem-ok-symbolic") },

                        add_prefix = &gtk::Spinner {
                            set_spinning: true,

                            #[watch]
                            set_visible: model.currently_installing == CurrentlyInstalling::Denpendecies,
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
                        set_visible: model.currently_installing != CurrentlyInstalling::Fonts && model.currently_installing != CurrentlyInstalling::Denpendecies,

                        #[watch]
                        set_text: Some(&model.progress_bar_message),
                        set_show_text: true,
                    },

                    gtk::Box {
                        set_halign: gtk::Align::Center,
                        set_orientation: gtk::Orientation::Horizontal,

                        gtk::Label {
                            #[watch]
                            set_visible: model.currently_installing == CurrentlyInstalling::Fonts || model.currently_installing == CurrentlyInstalling::Denpendecies,

                            #[watch]
                            set_label: &model.progress_bar_message,
                            add_css_class: "title-3",
                        },

                        gtk::Spinner {
                            set_spinning: true,
                            set_margin_start: 24,

                            #[watch]
                            set_visible: model.currently_installing == CurrentlyInstalling::Fonts || model.currently_installing == CurrentlyInstalling::Denpendecies,
                        }
                    }
                }
            },
        }
    }

    async fn init(
        _: Self::Init,
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

            installation_handler: manager::HandleComponentInstallation::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
            downloaded_component_name: String::new(),
            currently_installing: CurrentlyInstalling::None,
            msg_when_done: None,
        };

        let proton_combo = &model.proton_combo;
        let dxvk_combo = &model.dxvk_combo;

        let widgets = view_output!();

        AsyncComponentParts { widgets, model }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) -> () {
        match message {
            DownloadComponentsMsg::UpdateDownloadedComponentName(name) => {
                self.downloaded_component_name = name;
            }
            DownloadComponentsMsg::UpdateProgressBar((current, max_progress)) => {
                if self.currently_installing != CurrentlyInstalling::Fonts
                    && self.currently_installing != CurrentlyInstalling::Denpendecies
                {
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
                } else {
                    self.progress_bar_message =
                        format!("Downloading {}", self.downloaded_component_name);
                }
            }
            DownloadComponentsMsg::UpdateProgressBarMsg(message, message_when_done) => {
                self.progress_bar_message = message;
                self.msg_when_done = message_when_done;
            }
            DownloadComponentsMsg::ShowDoneMsg => {
                if let Some(msg) = self.msg_when_done.clone() {
                    self.progress_bar_message = msg;
                }
            }
            DownloadComponentsMsg::UpdateCurrentlyInstalling(currently_installing) => {
                self.currently_installing = currently_installing;
            }
            DownloadComponentsMsg::ShowError(message) => {
                let dialog = unsafe {
                    adw::MessageDialog::new(
                        MAIN_WINDOW.as_ref(),
                        Some("Error while downloading components"),
                        Some(&message),
                    )
                };

                dialog.add_response("close", "Close");
                dialog.add_response("copy", "Copy");

                dialog.set_response_appearance("copy", adw::ResponseAppearance::Suggested);

                dialog.connect_response(Some("copy"), move |_, _| {
                    if let Err(err) = Clipboard::new().unwrap().set_text(&message.clone()) {
                        error!("Failed to copy the error to the clipboard : {}", err);
                    }
                });

                dialog.present();
            }
            DownloadComponentsMsg::Finish => {
                let _ = sender.output(SetupPageMsg::Finish);
            }
            DownloadComponentsMsg::Quit => relm4::main_application().quit(),
        }

        if self.selected_proton_version.is_none()
            && self.selected_dxvk_version.is_none()
            && self.currently_installing != CurrentlyInstalling::None
        {
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
        self.sender.input(DownloadComponentsMsg::ShowDoneMsg);
        let mut guard = self.private.lock().unwrap();
        *guard = None;
    }
}
