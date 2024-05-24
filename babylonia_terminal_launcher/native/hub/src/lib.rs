//! This `hub` crate is the
//! entry point of the Rust logic.

// This `tokio` will be used by Rinf.
// You can replace it with the original `tokio`
// if you're not targeting the web.
use tokio_with_wasm::tokio;

mod config;
mod game_state;
mod github;
mod messages;

rinf::write_interface!();

// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    // Repeat `tokio::spawn` anywhere in your code
    // if more concurrent tasks are needed.
    //tokio::spawn(sample_functions::tell_numbers());
    //tokio::spawn(sample_functions::stream_fractal());
    //tokio::spawn(sample_functions::run_debug_tests());
    tokio::spawn(game_state::listen_game_state());
    tokio::spawn(config::listen_config());
    tokio::spawn(config::listen_config());
    tokio::spawn(github::listen_proton_version());
}
