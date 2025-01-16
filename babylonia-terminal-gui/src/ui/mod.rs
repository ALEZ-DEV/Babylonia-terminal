use std::convert::identity;

use crate::{manager, IS_DEVEL};
use babylonia_terminal_sdk::game_state::GameState;

use log::debug;
use relm4::{
    adw::{
        self,
        prelude::{PreferencesGroupExt, PreferencesPageExt},
        ApplicationWindow,
    },
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    loading_widgets::LoadingWidgets,
    prelude::{
        AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
        SimpleAsyncComponent,
    },
    view, Component, ComponentController, Controller, RelmApp, RelmWidgetExt, WorkerController,
};

use libadwaita::OverlaySplitView;

use crate::APP_RESOURCE_PATH;

pub fn run(app: RelmApp<MainWindowMsg>) {
    app.run_async::<MainWindow>(None);
}

#[derive(Debug)]
pub enum MainWindowMsg {
    ToggleMenuVisibility,
    SelectPage,
    SetIsGameRunning(bool),
}

struct MainWindow {
    game_state: GameState,
    game_handler: WorkerController<manager::HandleGameProcess>,
    is_game_running: bool,
    is_menu_visible: bool,
}

impl MainWindow {
    fn new(game_state: GameState, sender: &relm4::AsyncComponentSender<Self>) -> Self {
        MainWindow {
            game_state,
            is_menu_visible: false,
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
            add_css_class?: IS_DEVEL.then_some("devel"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                OverlaySplitView {
                    #[watch]
                    set_show_sidebar: model.is_menu_visible,
                    set_collapsed: true,

                    #[wrap(Some)]
                    set_content = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        adw::HeaderBar {
                            pack_start = &gtk::Button {
                                set_icon_name: "open-menu-symbolic",
                                connect_clicked => MainWindowMsg::ToggleMenuVisibility,
                            },
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_vexpand: true,
                            set_margin_horizontal: 50,
                            set_valign: gtk::Align::Center,

                            adw::PreferencesPage {
                                add = &adw::PreferencesGroup {
                                    gtk::Picture {
                                        set_resource: Some(&format!("{APP_RESOURCE_PATH}/icons/hicolor/scalable/apps/icon.png")),
                                        set_vexpand: true,
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
                        },
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
                            set_label: "Item 1",
                        },

                        gtk::Button {
                            set_margin_vertical: 5,
                            set_label: "Item 2"
                        },

                        gtk::Button {
                            set_margin_vertical: 5,
                            set_label: "Item 3",
                        },
                    },
                },

                //adw::Flap {
                //    #[watch]
                //    set_reveal_flap: model.is_menu_visible,
                //    set_margin_all: 0,
                //    set_fold_policy: adw::FlapFoldPolicy::Auto,

                //    #[wrap(Some)]
                //    set_flap =

                //    #[wrap(Some)]
                //    set_content =
                //},
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
            MainWindowMsg::SelectPage => println!("Tried to select a new page"),
            MainWindowMsg::SetIsGameRunning(value) => self.is_game_running = value,
        }
    }
}
