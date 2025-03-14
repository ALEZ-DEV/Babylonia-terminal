use std::convert::identity;

use crate::{manager, IS_DEVEL};
use babylonia_terminal_sdk::game_state::GameState;

use log::debug;
use pages::game::GamePageMsg;
use relm4::{
    self,
    component::AsyncConnector,
    gtk::{self, prelude::*, Widget},
    loading_widgets::LoadingWidgets,
    once_cell::sync::OnceCell,
    prelude::*,
    *,
};

use adw::prelude::*;
use libadwaita as adw;

use crate::APP_RESOURCE_PATH;

pub mod pages;

pub static mut MAIN_WINDOW: Option<adw::ApplicationWindow> = None;

pub fn run(app: RelmApp<MainWindowMsg>) {
    app.run_async::<MainWindow>(None);
}

#[derive(Debug)]
pub enum MainWindowMsg {
    ToggleMenuVisibility,
    SelectPage(Pages),
    UpdateGameState,
}

struct MainWindow {
    game_state: GameState,
    setup_page: AsyncController<pages::steps::SetupPage>,
    game_page: AsyncController<pages::game::GamePage>,
    settings_page: AsyncConnector<pages::settings::SettingsPage>,
    about_page: AsyncConnector<pages::about::AboutPage>,
    current_page: Pages,
    is_menu_visible: bool,
}

impl MainWindow {
    fn new(game_state: GameState, sender: &relm4::AsyncComponentSender<Self>) -> Self {
        let setup_page = pages::steps::SetupPage::builder()
            .launch(game_state.clone())
            .forward(sender.input_sender(), identity);

        let game_page = pages::game::GamePage::builder()
            .launch(game_state.clone())
            .forward(sender.input_sender(), identity);

        let about_page = pages::about::AboutPage::builder().launch(());

        let settings_page = pages::settings::SettingsPage::builder().launch(());

        MainWindow {
            game_state,
            setup_page,
            game_page,
            settings_page,
            about_page,
            current_page: Pages::GamePage,
            is_menu_visible: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pages {
    GamePage,
    SettingsPage,
    AboutPage,
}

#[relm4::component(async)]
impl SimpleAsyncComponent for MainWindow {
    type Input = MainWindowMsg;

    type Output = ();

    type Init = Option<GameState>;

    view! {
        #[root]
        main_window = adw::ApplicationWindow {
            set_default_size: (700, 560),

            add_css_class?: IS_DEVEL.then_some("devel"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::OverlaySplitView {
                    #[watch]
                    set_show_sidebar: model.is_menu_visible,
                    set_collapsed: true,

                    #[wrap(Some)]
                    set_content = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        adw::HeaderBar {
                            pack_start = &gtk::Button {
                                set_icon_name: "open-menu-symbolic",

                                #[watch]
                                set_visible: model.game_state.is_environment_ready(),

                                connect_clicked => MainWindowMsg::ToggleMenuVisibility,
                            },
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_vexpand: true,
                            set_margin_horizontal: 50,
                            set_valign: gtk::Align::Center,

                            #[watch]
                            set_visible: model.game_state.is_environment_ready(),

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_vexpand: true,

                                #[watch]
                                set_visible: model.current_page == Pages::GamePage,

                                model.game_page.widget(),
                            },

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_vexpand: true,

                                #[watch]
                                set_visible: model.current_page == Pages::SettingsPage,

                                model.settings_page.widget(),
                            },

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_vexpand: true,

                                #[watch]
                                set_visible: model.current_page == Pages::AboutPage,

                                model.about_page.widget(),
                            },
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            #[watch]
                            set_visible: !model.game_state.is_environment_ready(),

                            model.setup_page.widget(),
                        }
                    },

                    #[wrap(Some)]
                    set_sidebar = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_width_request: 250,
                        set_margin_all: 10,

                        gtk::Picture {
                            set_resource: Some(&format!("{APP_RESOURCE_PATH}/icons/hicolor/scalable/apps/icon.png")),
                            set_hexpand: true,
                            set_margin_horizontal: 30,
                            set_margin_top: 30,
                            set_margin_bottom: 10,
                        },

                        gtk::Label {
                            set_label: "Babylonia Terminal",
                            set_margin_top: 12,
                            add_css_class: "title-1",
                            set_margin_horizontal: 30,
                            set_margin_bottom: 10,
                        },

                        gtk::Button {
                            set_margin_vertical: 5,
                            set_label: "Game",

                            connect_clicked => MainWindowMsg::SelectPage(Pages::GamePage),
                        },

                        gtk::Button {
                            set_margin_vertical: 5,
                            set_label: "Settings",

                            connect_clicked => MainWindowMsg::SelectPage(Pages::SettingsPage),
                        },

                        gtk::Button {
                            set_margin_vertical: 5,
                            set_label: "About",

                            connect_clicked => MainWindowMsg::SelectPage(Pages::AboutPage),
                        },
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
                &sender,
            );
        } else {
            model = MainWindow::new(game_state.unwrap(), &sender);
        }

        let widgets = view_output!();

        unsafe {
            MAIN_WINDOW = Some(widgets.main_window.clone());
        }

        debug!("current GameState : {:?}", model.game_state);

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
            MainWindowMsg::ToggleMenuVisibility => self.is_menu_visible = !self.is_menu_visible,
            MainWindowMsg::SelectPage(page) => {
                self.current_page = page;
                self.is_menu_visible = false;
            }
            MainWindowMsg::UpdateGameState => {
                self.game_state = GameState::get_current_state().await.unwrap();
                self.game_page.sender().send(GamePageMsg::UpdateGameState);
                debug!(
                    "is_environment_ready : {}",
                    self.game_state.is_environment_ready()
                );
            }
        }
    }
}
