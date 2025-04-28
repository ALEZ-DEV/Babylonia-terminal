use clap::Parser;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

fn main() {
    let args = babylonia_terminal_cli::arguments::Args::parse();

    let simple_logger = SimpleLogger::new()
        .with_module_level("hyper", LevelFilter::Off)
        .with_module_level("hyper_util", LevelFilter::Off)
        .with_module_level("tracing", LevelFilter::Off)
        .with_module_level("rustls", LevelFilter::Off)
        .with_module_level("minreq", LevelFilter::Off)
        .with_module_level("tokio_utils", LevelFilter::Off);

    if args.debug || cfg!(debug_assertions) {
        simple_logger.init().unwrap();
        info!("Debug messages enabled");
    } else {
        simple_logger.with_level(LevelFilter::Info).init().unwrap();
    }

    if args.gui {
        babylonia_terminal_gui::run();
    } else {
        babylonia_terminal_cli::run();
    }
}
