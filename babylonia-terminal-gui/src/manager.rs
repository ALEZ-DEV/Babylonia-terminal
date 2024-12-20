use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_config::GameConfig,
    game_manager::GameManager,
};
use log::error;
use relm4::{
    tokio::{self, sync::OnceCell},
    Worker,
};
use wincompatlib::prelude::Proton;

use crate::ui;

static PROTON: OnceCell<Proton> = OnceCell::const_new();

pub async fn get_proton() -> Proton {
    PROTON
        .get_or_init(|| async {
            let proton_component = ProtonComponent::new(GameConfig::get_config().await.config_dir);
            let proton = proton_component.init_proton();
            if let Err(ref e) = proton {
                error!("Failed to initialize proton : {}", e);
            }
            proton.unwrap()
        })
        .await
        .clone()
}

pub async fn run_game() {
    let proton = get_proton().await;
    let game_dir = GameConfig::get_game_dir().await;
    if game_dir.is_none() {
        error!("Failed to start game, the game directory was not found")
    }

    GameManager::start_game(&proton, game_dir.unwrap(), None, false).await;
}

#[derive(Debug)]
pub enum HandleGameProcessMsg {
    RunGame,
}

pub struct HandleGameProcess;

impl Worker for HandleGameProcess {
    type Init = ();

    type Input = HandleGameProcessMsg;

    type Output = ui::MainWindowMsg;

    fn init(_init: Self::Init, _sender: relm4::ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            HandleGameProcessMsg::RunGame => {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        sender.output(ui::MainWindowMsg::SetIsGameRunning(true));
                        run_game().await;
                        sender.output(ui::MainWindowMsg::SetIsGameRunning(false));
                    });
            }
        }
    }
}
