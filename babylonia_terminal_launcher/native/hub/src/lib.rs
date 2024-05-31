use tokio_with_wasm::tokio;

mod config;
mod dxvk;
mod game_state;
mod github;
mod messages;
mod proton;

rinf::write_interface!();

async fn main() {
    //config
    tokio::spawn(game_state::listen_game_state());
    tokio::spawn(config::listen_config());

    //github
    tokio::spawn(github::listen_proton_version());
    tokio::spawn(github::listen_dxvk_version());

    //installation
    tokio::spawn(proton::listen_proton_installation());
    tokio::spawn(dxvk::listen_dxvk_installation());
}
