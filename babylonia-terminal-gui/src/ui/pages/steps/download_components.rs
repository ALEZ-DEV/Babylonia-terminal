use babylonia_terminal_sdk::{
    components::{
        dxvk_component::{self, DXVKComponent},
        proton_component::{self, ProtonComponent},
    },
    utils::github_requester::{GithubRelease, GithubRequester},
};
use relm4::{
    self,
    gtk::{self, prelude::*},
    prelude::*,
    *,
};

use adw::prelude::*;
use libadwaita as adw;

use crate::ui::MainWindowMsg;

use super::SetupPageMsg;

#[derive(Debug)]
pub enum DownloadComponentsMsg {
    Next,
}

pub struct DownloadComponentsPage {
    proton_versions: Vec<GithubRelease>,
    dxvk_versions: Vec<GithubRelease>,
}

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

                    set_model: Some(&gtk::StringList::new(model
                        .proton_versions
                        .iter()
                        .map(|r| r.tag_name.as_str())
                        .collect::<Vec<&str>>()
                        .as_slice())),
                },

                adw::ComboRow {
                    set_title: "DXVK version",

                    set_model: Some(&gtk::StringList::new(model
                        .dxvk_versions
                        .iter()
                        .map(|r| r.tag_name.as_str())
                        .collect::<Vec<&str>>()
                        .as_slice())),
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
        let proton_releases = ProtonComponent::get_github_releases(
            proton_component::PROTON_DEV,
            proton_component::PROTON_REPO,
        )
        .await
        .unwrap(); //TODO: remove unwrap()

        let dxvk_releases =
            DXVKComponent::get_github_releases(dxvk_component::DXVK_DEV, dxvk_component::DXVK_REPO)
                .await
                .unwrap(); //TODO: remove unwrap()

        let model = DownloadComponentsPage {
            proton_versions: proton_releases,
            dxvk_versions: dxvk_releases,
        };

        let widgets = view_output!();

        AsyncComponentParts { widgets, model }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) -> () {
        todo!();
    }
}
