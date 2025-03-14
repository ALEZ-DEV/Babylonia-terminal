use relm4::{
    gtk::prelude::{OrientableExt, WidgetExt},
    prelude::{gtk, AsyncComponentParts, SimpleAsyncComponent},
};

pub struct SettingsPage;

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for SettingsPage {
    type Input = ();

    type Output = ();

    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_label: "This page is under construction!",
                add_css_class: "title-1",
            },

            gtk::Label {
                set_margin_top: 24,

                set_label: "Please wait patiently :)",
                add_css_class: "title-4",
            },
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: relm4::AsyncComponentSender<Self>,
    ) -> relm4::prelude::AsyncComponentParts<Self> {
        let model = SettingsPage {};
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
