use std::{path::PathBuf, str::FromStr};

use babylonia_terminal_sdk::{game_config::GameConfig, game_manager::GameManager};
use log::info;
use relm4::{
    gtk,
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    view, RelmWidgetExt,
};

use libadwaita::{self as adw, prelude::*};

use super::SetupPageMsg;

#[derive(Debug)]
pub enum ChooseGameDirectoryMsg {
    ChoosePath,
    Next,
}

pub struct ChooseGameDirectoryPage {
    path: PathBuf,
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for ChooseGameDirectoryPage {
    type Input = ChooseGameDirectoryMsg;

    type Output = SetupPageMsg;

    type Init = ();

    view! {
        #[root]
        gtk::Box {
            adw::PreferencesPage {
                set_hexpand: true,

                add = &adw::PreferencesGroup {
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    gtk::Label {
                        set_label: "Game directory",
                        add_css_class: "title-1"
                    },
                },

                add = &adw::PreferencesGroup {
                    set_valign: gtk::Align::Center,
                    set_vexpand: true,

                    adw::ActionRow {
                        set_title: "Game directory",
                        set_icon_name: Some("folder-symbolic"),
                        set_activatable: true,

                        #[watch]
                        set_subtitle: model.path.to_str().unwrap(),

                        connect_activated => ChooseGameDirectoryMsg::ChoosePath,
                    },
                },

                add = &adw::PreferencesGroup {
                    set_margin_vertical: 48,

                    gtk::Button {
                        set_css_classes: &["suggested-action", "pill"],

                        set_label: "Next",
                        set_hexpand: false,
                        set_width_request: 200,

                        connect_clicked => ChooseGameDirectoryMsg::Next,
                    },
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let path = if let Some(dir) = GameConfig::get_config().await.game_dir {
            dir
        } else {
            GameConfig::get_config_directory().await
        };

        let model = ChooseGameDirectoryPage { path };

        let widgets = view_output!();

        AsyncComponentParts { widgets, model }
    }

    async fn update(&mut self, message: Self::Input, sender: relm4::AsyncComponentSender<Self>) {
        match message {
            ChooseGameDirectoryMsg::ChoosePath => {
                info!("choose path");
                let result = rfd::AsyncFileDialog::new()
                    .set_directory(self.path.clone())
                    .pick_folder()
                    .await;

                if let Some(result) = result {
                    self.path = result.path().to_path_buf();
                }

                GameConfig::set_game_dir(Some(self.path.clone()))
                    .await
                    .unwrap(); // TODO: remove unwrap
            }
            ChooseGameDirectoryMsg::Next => {
                let _ = sender.output(SetupPageMsg::GoToDownloadComponentPage);
            }
        }
    }
}
