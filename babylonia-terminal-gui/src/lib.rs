use log::debug;
use relm4::{
    gtk::{self, gdk, gio},
    RelmApp,
};

mod manager;
mod ui;

pub const APP_RESOURCE_PATH: &str = "/moe/celica/babylonia-terminal";

pub fn run() {
    debug!("Start GUI!");
    let app = RelmApp::new("moe.celica.BabyloniaTerminal").with_args(vec![]);

    gio::resources_register_include!("resources.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path(&format!("{APP_RESOURCE_PATH}/icons"));
    ui::run(app);
}
