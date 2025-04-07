use std::{convert::identity, fmt::format};

use arboard::Clipboard;
use babylonia_terminal_sdk::game_state::GameState;
use libadwaita::prelude::{MessageDialogExt, PreferencesPageExt};
use log::error;
use relm4::{
    adw,
    gtk::{
        self,
        prelude::{ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    Component, RelmWidgetExt, WorkerController,
};

use crate::{
    manager,
    ui::{MainWindowMsg, MAIN_WINDOW},
    APP_RESOURCE_PATH,
};

#[derive(Debug)]
pub struct GamePage {
    game_state: GameState,
    game_handler: WorkerController<manager::HandleGameProcess>,
    installation_handler: WorkerController<manager::HandleGameInstallation>,
    is_game_running: bool,
    is_downloading: bool,
    is_patching: bool,
    progress_bar_reporter: std::sync::Arc<ProgressBarGameInstallationReporter>,
    progress_bar_message: String,
    fraction: f64,
}

#[derive(Debug)]
pub enum GamePageMsg {
    SetIsGameRunning(bool),
    SetIsDownloading(bool),
    SetIsPatching(bool),
    UpdateGameState,
    UpdateProgressBar(u64, u64),
    ShowError(String),
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for GamePage {
    type Input = GamePageMsg;

    type Output = MainWindowMsg;

    type Init = GameState;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    gtk::Picture {
                        set_resource: Some(&format!("{APP_RESOURCE_PATH}/icons/hicolor/scalable/apps/icon.png")),
                        set_vexpand: true,
                        set_width_request: 350,
                    },

                    gtk::Label {
                        set_label: "Babylonia Terminal",
                        set_margin_top: 24,
                        add_css_class: "title-1",
                    },
                },

                add = &adw::PreferencesGroup {
                    set_margin_vertical: 48,

                    gtk::Button {
                        set_css_classes: &["suggested-action", "pill"],

                        set_label: "Install",
                        set_hexpand: false,
                        set_width_request: 200,

                        #[watch]
                        set_visible: model.game_state == GameState::GameNotInstalled,

                        #[watch]
                        set_sensitive: !model.is_downloading,
                        connect_clicked[sender = model.installation_handler.sender().clone(), progress_bar = model.progress_bar_reporter.clone()] => move |_| {
                            sender.send(manager::HandleGameInstallationMsg::StartInstallation(progress_bar.clone())).unwrap();
                        },
                    },

                    gtk::Button {
                        set_css_classes: &["suggested-action", "pill"],

                        set_label: "Apply patch",
                        set_hexpand: false,
                        set_width_request: 200,

                        #[watch]
                        set_visible: model.game_state == GameState::GameNotPatched,

                        #[watch]
                        set_sensitive: !model.is_patching,
                        connect_clicked[sender = model.installation_handler.sender().clone()] => move |_| {
                            sender.send(manager::HandleGameInstallationMsg::StartPatch).unwrap();
                        },
                    },

                    gtk::Button {
                        set_css_classes: &["suggested-action", "pill"],

                        set_label: "Start game",
                        set_hexpand: false,
                        set_width_request: 200,

                        #[watch]
                        set_visible: model.game_state == GameState::GameInstalled,

                        #[watch]
                        set_sensitive: !model.is_game_running,
                        connect_clicked[sender = model.game_handler.sender().clone()] => move |_| {
                            sender.send(manager::HandleGameProcessMsg::RunGame).unwrap();
                        },
                    },

                    gtk::Button {
                        set_css_classes: &["suggested-action", "pill"],

                        set_label: "Update",
                        set_hexpand: false,
                        set_width_request: 200,

                        #[watch]
                        set_visible: model.game_state == GameState::GameNeedUpdate,

                        #[watch]
                        set_sensitive: !model.is_downloading,
                        connect_clicked[sender = model.installation_handler.sender().clone(), progress_bar = model.progress_bar_reporter.clone()] => move |_| {
                            sender.send(manager::HandleGameInstallationMsg::StartUpdate(progress_bar.clone())).unwrap();
                        },
                    },
                },
            },

            gtk::ProgressBar {
                #[watch]
                set_fraction: model.fraction,

                #[watch]
                set_visible: (model.game_state == GameState::GameNotInstalled || model.game_state == GameState::GameNeedUpdate) && model.is_downloading && model.fraction != 0.0,

                #[watch]
                set_text: Some(&model.progress_bar_message),
                set_show_text: true,
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                #[watch]
                set_visible: (model.game_state == GameState::GameNotInstalled || model.game_state == GameState::GameNeedUpdate) && model.is_downloading && model.fraction == 0.0,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_margin_bottom: 24,

                    gtk::Label {
                        set_label: "Checking game files",

                        add_css_class: "title-2",
                    },

                    gtk::Spinner {
                        set_spinning: true,
                        set_margin_start: 24,
                    }
                },

                gtk::Label {
                    set_label: "This can take some time, please wait...",

                    add_css_class: "title-4",
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                #[watch]
                set_visible: model.game_state == GameState::GameNotPatched && model.is_patching,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,

                    gtk::Label {
                        set_label: "Patching game",

                        add_css_class: "title-2",
                    },

                    gtk::Spinner {
                        set_spinning: true,
                    }
                },

                gtk::Label {
                    set_label: "This can take some time, please wait...",

                    add_css_class: "title-4",
                },
            }
        }
    }

    async fn init(
        game_state: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let model = GamePage {
            progress_bar_reporter: ProgressBarGameInstallationReporter::create(sender.clone()),
            game_state,
            game_handler: manager::HandleGameProcess::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
            installation_handler: manager::HandleGameInstallation::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
            is_game_running: false,
            is_downloading: false,
            is_patching: false,
            fraction: 0.0,
            progress_bar_message: String::new(),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _: relm4::AsyncComponentSender<Self>) -> () {
        match message {
            GamePageMsg::SetIsGameRunning(value) => self.is_game_running = value,
            GamePageMsg::SetIsDownloading(value) => self.is_downloading = value,
            GamePageMsg::SetIsPatching(value) => self.is_patching = value,
            GamePageMsg::UpdateGameState => {
                self.game_state = GameState::get_current_state().await.unwrap()
            } //TODO: remove unwrap()
            GamePageMsg::UpdateProgressBar(current, max_progress) => {
                self.fraction = if current == 0 {
                    0.0
                } else {
                    current as f64 / max_progress as f64
                };

                //1024^3 = 1073741824
                let current_gb = current as f64 / 1073741824 as f64;
                let max_gb = max_progress as f64 / 1073741824 as f64;

                self.progress_bar_message = format!(
                    "Downloading : {:.2}% ({:.2} / {:.2}GiB)",
                    self.fraction * 100 as f64,
                    current_gb,
                    max_gb,
                );
            }
            GamePageMsg::ShowError(message) => {
                let dialog = unsafe {
                    adw::MessageDialog::new(
                        MAIN_WINDOW.as_ref(),
                        Some("Something went wrong"),
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
        }
    }
}

#[derive(Debug)]
struct ProgressBarGameInstallationReporterPrivate {
    max_progress: Option<u64>,
}

#[derive(Debug)]
pub struct ProgressBarGameInstallationReporter {
    private: std::sync::Mutex<Option<ProgressBarGameInstallationReporterPrivate>>,
    sender: relm4::AsyncComponentSender<GamePage>,
}

impl ProgressBarGameInstallationReporter {
    fn create(page: relm4::AsyncComponentSender<GamePage>) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
            sender: page,
        })
    }
}

impl downloader::progress::Reporter for ProgressBarGameInstallationReporter {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let private = ProgressBarGameInstallationReporterPrivate { max_progress };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            self.sender.input(GamePageMsg::UpdateProgressBar(
                current,
                p.max_progress.unwrap(),
            ));
        }
    }

    fn set_message(&self, _: &str) {}

    fn done(&self) {
        let mut guard = self.private.lock().unwrap();
        *guard = None;
    }
}
