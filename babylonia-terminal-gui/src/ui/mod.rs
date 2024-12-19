use std::convert::identity;

use crate::manager;
use babylonia_terminal_sdk::game_state::GameState;

use relm4::{
    adw::{self, ApplicationWindow},
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    loading_widgets::LoadingWidgets,
    prelude::{AsyncComponentParts, SimpleAsyncComponent},
    view, Component, RelmApp, RelmWidgetExt, WorkerController,
};

use crate::APP_RESOURCE_PATH;

pub fn run(app: RelmApp<MainWindowMsg>) {
    app.run_async::<MainWindow>(None);
}

#[derive(Debug)]
pub enum MainWindowMsg {
    SetIsGameRunning(bool),
}

struct MainWindow {
    game_handler: WorkerController<manager::HandleGameProcess>,
    game_state: GameState,
    is_game_running: bool,
}

impl MainWindow {
    fn new(game_state: GameState, sender: relm4::AsyncComponentSender<Self>) -> Self {
        MainWindow {
            game_state,
            is_game_running: false,
            game_handler: manager::HandleGameProcess::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
        }
    }
}

#[relm4::component(async)]
impl SimpleAsyncComponent for MainWindow {
    type Input = MainWindowMsg;

    type Output = ();

    type Init = Option<GameState>;

    view! {
        #[root]
        adw::ApplicationWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar,

                gtk::Image {
                    set_resource: Some(&format!("{APP_RESOURCE_PATH}/icons/hicolor/scalable/apps/icon.png")),
                    set_height_request: 256,
                },

                #[name(start_button)]
                gtk::Button {
                    #[watch]
                    set_sensitive: !model.is_game_running,
                    set_label: "Start game",
                    connect_clicked[sender = model.game_handler.sender().clone()] => move |_| {
                        sender.send(manager::HandleGameProcessMsg::RunGame).unwrap();
                    },
                },
            }
        }
    }

    async fn init(
        game_state: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model;
        if game_state.is_none() {
            model = MainWindow::new(
                babylonia_terminal_sdk::game_state::GameState::get_current_state()
                    .await
                    .unwrap(),
                sender,
            );
        } else {
            model = MainWindow::new(game_state.unwrap(), sender);
        }

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local]
            root {
                set_title: Some("Babylonia Terminal"),
                set_default_width: 700,
                set_default_height: 300,

                #[name(spinner)]
                gtk::Spinner {
                    start: (),
                    set_halign: gtk::Align::Center,
                }
            }
        }

        Some(LoadingWidgets::new(root, spinner))
    }

    async fn update(&mut self, message: Self::Input, _sender: relm4::AsyncComponentSender<Self>) {
        match message {
            MainWindowMsg::SetIsGameRunning(value) => self.is_game_running = value,
        }
    }
}
