use babylonia_terminal_sdk::{
    components::proton_component::ProtonComponent, game_config::GameConfig,
    game_manager::GameManager,
};
use log::error;
use relm4::tokio::sync::OnceCell;
use wincompatlib::prelude::Proton;

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
