use log::debug;
use relm4::RelmApp;

mod manager;
mod ui;

pub fn run() {
    debug!("Start GUI!");
    let app = RelmApp::new("moe.celica.BabyloniaTerminal").with_args(vec![]);
    ui::run(app);
}
