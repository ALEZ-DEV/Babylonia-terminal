use log::debug;
use relm4::RelmApp;
use relm4::{
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt},
    },
    ComponentParts, RelmWidgetExt, SimpleComponent,
};

pub fn run() {
    debug!("Start GUI!");
    let app = RelmApp::new("moe.celica.BabyloniaTerminal").with_args(vec![]);
    app.run::<AppModel>(0);
}

#[derive(Debug)]
pub enum AppMsg {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppMsg;

    type Output = ();

    type Init = u8;

    view! {
        gtk::Window {
            set_title: Some("Babylonia Terminal"),
            set_default_width: 700,
            set_default_height: 300,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked => AppMsg::Increment,
                },

                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked => AppMsg::Decrement,
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter : {}", model.counter),
                    set_margin_all: 5,
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}
