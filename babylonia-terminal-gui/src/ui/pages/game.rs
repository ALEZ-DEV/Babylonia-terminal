use std::convert::identity;

use libadwaita::prelude::PreferencesPageExt;
use relm4::{
    adw,
    gtk::{
        self,
        prelude::{ButtonExt, WidgetExt},
    },
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    Component, RelmWidgetExt, WorkerController,
};

use crate::{manager, ui::MainWindowMsg, APP_RESOURCE_PATH};

//use adw::prelude::*;
//use libadwaita as adw;

pub struct GamePage {
    game_handler: WorkerController<manager::HandleGameProcess>,
    is_game_running: bool,
}

#[derive(Debug)]
pub enum GamePageMsg {
    SetIsGameRunning(bool),
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for GamePage {
    type Input = GamePageMsg;

    type Output = MainWindowMsg;

    type Init = ();

    view! {
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

                    set_label: "Start game",
                    set_hexpand: false,
                    set_width_request: 200,

                    #[watch]
                    set_sensitive: !model.is_game_running,
                    connect_clicked[sender = model.game_handler.sender().clone()] => move |_| {
                        sender.send(manager::HandleGameProcessMsg::RunGame).unwrap();
                    },
                },
            },
        },
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let model = GamePage {
            game_handler: manager::HandleGameProcess::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
            is_game_running: false,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _: relm4::AsyncComponentSender<Self>) -> () {
        match message {
            GamePageMsg::SetIsGameRunning(value) => self.is_game_running = value,
        }
    }
}
