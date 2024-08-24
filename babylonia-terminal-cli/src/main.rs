use babylonia_terminal_sdk::game_config::GameConfig;
use clap::Parser;
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;

mod arguments;
pub mod game;
pub mod reporter;
pub mod utils;

use crate::arguments::Args;

#[tokio::main]
async fn main() {
    let simple_logger = SimpleLogger::new()
        .with_module_level("hyper", LevelFilter::Off)
        .with_module_level("hyper_util", LevelFilter::Off)
        .with_module_level("tracing", LevelFilter::Off)
        .with_module_level("rustls", LevelFilter::Off)
        .with_module_level("minreq", LevelFilter::Off)
        .with_module_level("tokio_utils", LevelFilter::Off);

    if cfg!(debug_assertions) {
        simple_logger.init().unwrap();
    } else {
        simple_logger.with_level(LevelFilter::Info).init().unwrap();
    }

    let args = Args::parse();
    debug!("Launch option -> {:?}", args.options);

    if let Some(command) = args.set_options {
        GameConfig::set_launch_options(command)
            .await
            .expect("Failed to save launch options into the config file");
    }

    game::run(args.options, args.logs).await;
}
