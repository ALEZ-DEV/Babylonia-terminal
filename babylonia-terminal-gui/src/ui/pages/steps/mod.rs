use std::convert::identity;

use babylonia_terminal_sdk::{game_config::GameConfig, game_state::GameState};
use choose_game_directory::ChooseGameDirectoryPage;
use download_components::DownloadComponentsPage;
use libadwaita::prelude::{OrientableExt, WidgetExt};
use relm4::{
    prelude::{
        adw, gtk, AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
        SimpleAsyncComponent,
    },
    AsyncComponentSender, Component, ComponentController, Controller,
};
use welcome::WelcomePage;

use crate::ui::MainWindowMsg;

mod choose_game_directory;
pub mod download_components;
mod welcome;

#[derive(Debug)]
pub enum SetupPageMsg {
    GoToChooseGameDirectoryPage,
    GoToDownloadComponentPage,
    Finish,
}

pub struct SetupPage {
    game_state: GameState,
    welcome_page: Controller<welcome::WelcomePage>,
    choose_game_directory_page: AsyncController<choose_game_directory::ChooseGameDirectoryPage>,
    download_components_page: AsyncController<download_components::DownloadComponentsPage>,

    carousel: adw::Carousel,
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for SetupPage {
    type Input = SetupPageMsg;

    type Output = MainWindowMsg;

    type Init = GameState;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_vexpand: true,

            #[local_ref]
            carousel -> adw::Carousel {
                set_allow_mouse_drag: false,
                set_allow_long_swipes: false,
                set_allow_scroll_wheel: false,

                append = model.welcome_page.widget(),
                append = model.choose_game_directory_page.widget(),
                append = model.download_components_page.widget(),
            },

            adw::CarouselIndicatorDots {
                set_carousel: Some(&carousel),
                set_height_request: 32,
            },
        },
    }

    async fn init(
        game_state: GameState,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let welcome_page = WelcomePage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let choose_game_directory_page = ChooseGameDirectoryPage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let download_components_page = DownloadComponentsPage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let carousel = adw::Carousel::new();

        carousel.scroll_to(welcome_page.widget(), true);

        let model = SetupPage {
            welcome_page,
            choose_game_directory_page,
            download_components_page,
            game_state,
            carousel: carousel.clone(),
        };
        let widgets = view_output!();

        AsyncComponentParts { widgets, model }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        self.game_state = GameState::get_current_state().await.unwrap(); // TODO: delete this unwrap()

        match message {
            SetupPageMsg::GoToChooseGameDirectoryPage => {
                self.carousel
                    .scroll_to(self.choose_game_directory_page.widget(), true);
            }
            SetupPageMsg::GoToDownloadComponentPage => {
                self.carousel
                    .scroll_to(self.download_components_page.widget(), true);
            }
            SetupPageMsg::Finish => {
                sender.output(MainWindowMsg::UpdateGameState);
            }
        }
    }
}
