use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
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

    let args = babylonia_terminal_cli::arguments::Args::parse();

    if args.gui {
        babylonia_terminal_gui::run();
    } else {
        babylonia_terminal_cli::run();
    }
}
