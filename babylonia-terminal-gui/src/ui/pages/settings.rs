use arboard::Clipboard;
use babylonia_terminal_sdk::{game_config::GameConfig, game_manager::EnvironmentVariable};
use log::error;
use relm4::{
    gtk::{
        prelude::{EditableExt, GtkWindowExt, OrientableExt, WidgetExt},
        InputHints,
    },
    prelude::{gtk, AsyncComponentParts, SimpleAsyncComponent},
};

use libadwaita::{
    self as adw,
    prelude::{
        EntryRowExt, MessageDialogExt, PreferencesGroupExt, PreferencesPageExt, PreferencesRowExt,
    },
};

use crate::ui::MAIN_WINDOW;

#[derive(Debug)]
pub enum SettingsPageMsg {
    UpdateLaunchOption(Option<String>),
    ShowError(String),
}

pub struct SettingsPage {
    launch_option: String,
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for SettingsPage {
    type Input = SettingsPageMsg;

    type Output = ();

    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::PreferencesPage {
                set_title: "General settings",

                add = &adw::PreferencesGroup {
                    set_width_request: 500,
                    set_title: "Launch option",
                    set_description: Some("Pass launch options to tinker the behavior of the game"),

                    adw::EntryRow {
                        set_title: "%command%",
                        set_text: &model.launch_option,

                        connect_changed[sender] => move |entry| {
                            let command = entry.text().trim().to_string();
                            if command.is_empty() {
                                sender.input(SettingsPageMsg::UpdateLaunchOption(None))
                            } else {
                                sender.input(SettingsPageMsg::UpdateLaunchOption(Some(command)))
                            }
                        }
                    }
                },

                add = &adw::PreferencesGroup {
                    set_width_request: 500,
                    set_title: "Environment variables",
                    set_description: Some("Pass environment variables to tinker the behavior of the game"),

                }
            }
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let launch_option = match GameConfig::get_launch_options().await {
            Err(e) => String::new(),
            Ok(v) => match v {
                None => String::new(),
                Some(launch) => launch,
            },
        };

        let model = SettingsPage { launch_option };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: relm4::AsyncComponentSender<Self>) {
        match message {
            SettingsPageMsg::UpdateLaunchOption(new_launch_option) => {
                if let Err(e) = GameConfig::set_launch_options(new_launch_option).await {
                    sender.input(SettingsPageMsg::ShowError(format!(
                        "Something went wrong when updated the launch options : {}",
                        e
                    )));
                }
            }
            SettingsPageMsg::ShowError(message) => {
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

struct EnvVarWidget {
    env_var: EnvironmentVariable,
}
