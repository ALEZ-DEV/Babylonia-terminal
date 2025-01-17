use libadwaita::prelude::{ButtonExt, PreferencesPageExt, PreferencesRowExt, WidgetExt};
use relm4::prelude::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::{adw, gtk, AsyncComponentSender, RelmWidgetExt};

use crate::ui::MainWindowMsg;

use super::SetupPageMsg;

#[derive(Debug)]
pub enum DownloadComponentsMsg {
    Next,
}

pub struct DownloadComponentsPage;

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DownloadComponentsPage {
    type Input = DownloadComponentsMsg;

    type Output = SetupPageMsg;

    type Init = ();

    view! {
        #[root]
        adw::PreferencesPage {
            set_hexpand: true,

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

                adw::ComboRow {
                    set_title: "Proton version",
                },

                adw::ComboRow {
                    set_title: "DXVK version",
                },
            },

            add = &adw::PreferencesGroup {
                set_margin_vertical: 48,

                gtk::Button {
                    set_css_classes: &["suggested-action", "pill"],

                    set_label: "Next",
                    set_hexpand: false,
                    set_width_request: 200,

                  connect_clicked => DownloadComponentsMsg::Next,
                },
            },
        },
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = DownloadComponentsPage;
        let widgets = view_output!();

        AsyncComponentParts { widgets, model }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) -> () {
        todo!();
    }
}
