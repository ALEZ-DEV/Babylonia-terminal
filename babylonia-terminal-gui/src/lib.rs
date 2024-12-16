use babylonia_terminal_sdk::game_state::GameState;
use log::debug;
use manager::run_game;
use relm4::{
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    loading_widgets::LoadingWidgets,
    prelude::{AsyncComponent, AsyncComponentParts, SimpleAsyncComponent},
    view, ComponentParts, RelmWidgetExt, SimpleComponent,
};
use relm4::{Component, RelmApp};

mod manager;

pub fn run() {
    debug!("Start GUI!");
    let app = RelmApp::new("moe.celica.BabyloniaTerminal").with_args(vec![]);
    app.run_async::<MainWindow>(None);
}

#[derive(Debug)]
pub enum MainWindowMsg {
    RunGame,
}

struct MainWindow {
    game_state: GameState,
    is_game_running: bool,
}

impl MainWindow {
    fn new(game_state: GameState) -> Self {
        MainWindow {
            game_state,
            is_game_running: false,
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
        gtk::Window {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                #[name(start_button)]
                gtk::Button {
                    set_label: "Start game",
                    connect_clicked => MainWindowMsg::RunGame,
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
            );
        } else {
            model = MainWindow::new(game_state.unwrap());
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
            MainWindowMsg::RunGame => run_game().await,
        }
    }
}
