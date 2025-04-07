use libadwaita::prelude::{ButtonExt, PreferencesPageExt, WidgetExt};
use relm4::{adw, gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

use super::SetupPageMsg;

use crate::APP_RESOURCE_PATH;

#[derive(Debug)]
pub enum WelcomePageMsg {
    Next,
}

pub struct WelcomePage;

#[relm4::component(pub)]
impl SimpleComponent for WelcomePage {
    type Input = WelcomePageMsg;

    type Output = SetupPageMsg;

    type Init = ();

    view! {
        #[root]
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Picture {
                    set_resource: Some(&format!("{APP_RESOURCE_PATH}/icons/hicolor/scalable/apps/icon.png")),
                    set_vexpand: true,
                },

                gtk::Label {
                    set_label: "Babylonia Terminal",
                    set_margin_top: 24,
                    add_css_class: "title-1",
                },

                gtk::Label {
                    set_label: "We need to do some setup before to be able to play",

                    set_justify: gtk::Justification::Center,
                    set_wrap: true,
                    set_margin_top: 32
                },
            },

            add = &adw::PreferencesGroup {
                set_margin_vertical: 48,

                gtk::Button {
                    set_css_classes: &["suggested-action", "pill"],

                    set_label: "Next",
                    set_hexpand: false,
                    set_width_request: 200,

                    connect_clicked => WelcomePageMsg::Next,
                },
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WelcomePage {};
        let widgets = view_output!();

        ComponentParts { widgets, model }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            WelcomePageMsg::Next => sender
                .output(SetupPageMsg::GoToChooseGameDirectoryPage)
                .unwrap(),
        }
    }
}
