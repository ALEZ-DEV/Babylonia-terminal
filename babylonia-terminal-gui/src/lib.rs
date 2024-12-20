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

    relm4::set_global_css(&format!(
        "
        progressbar > text {{
            margin-bottom: 4px;
        }}

        window.classic-style {{
            background-repeat: no-repeat;
            background-size: cover;
        }}

        window.classic-style progressbar {{
            background-color: #00000020;
            border-radius: 16px;
            padding: 8px 16px;
        }}

        window.classic-style progressbar:hover {{
            background-color: #00000060;
            color: #ffffff;
            transition-duration: 0.5s;
            transition-timing-function: linear;
        }}

        .round-bin {{
            border-radius: 24px;
        }}
    "
    ));

    ui::run(app);
}
