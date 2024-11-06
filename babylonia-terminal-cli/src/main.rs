use babylonia_terminal_sdk::game_config::GameConfig;
use babylonia_terminal_sdk::utils;
use clap::Parser;
use log::{debug, info, LevelFilter};
use simple_logger::SimpleLogger;

mod arguments;
mod commands;
pub mod game;
pub mod reporter;
pub mod utils;

use crate::arguments::Args;
use crate::commands::Commands;

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

    match args.command {
        Some(Commands::SetLaunchOptions { launch_options }) => {
            GameConfig::set_launch_options(Some(launch_options))
                .await
                .expect("Failed to save launch options into the config file");
            info!("Successfully updated launch options!");
        }
        Some(Commands::SetGamePath { new_game_directory }) => {
            utils::fs::move_all_file_in_dir(
                "~/.babylonia-terminal/PGR",
                "~/.babylonia-terminal/PGR",
            );
        }
        None => game::run(args.options, args.logs).await,
    }
}
