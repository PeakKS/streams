use adw::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, WidgetClassExt};
use gtk::subclass::prelude::ApplicationWindowImpl;
use gtk::subclass::widget::{CompositeTemplateClass, WidgetImpl};
use gtk::subclass::window::{WindowImpl, WindowImplExt};
use gtk::{gio, glib, TemplateChild};

use crate::accounts::twitch;
use crate::application::StreamsApplication;

mod imp {
    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use gtk::prelude::ButtonExt;
    use gtk::subclass::widget::CompositeTemplateInitializingExt;

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/stream/streamsapp/Streams/ui/window.ui")]
    pub struct StreamsWindow {
        #[template_child]
        pub split_view: TemplateChild<adw::OverlaySplitView>,

        #[template_child]
        pub stream_tab_bar: TemplateChild<adw::TabBar>,

        #[template_child]
        pub stream_tab_view: TemplateChild<adw::TabView>,

        #[template_child]
        pub sign_in_button: TemplateChild<gtk::Button>,
    }

    impl Default for StreamsWindow {
        fn default() -> Self {
            Self {
                split_view: TemplateChild::default(),
                stream_tab_bar: TemplateChild::default(),
                stream_tab_view: TemplateChild::default(),
                sign_in_button: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StreamsWindow {
        const NAME: &'static str = "StreamsWindow";
        type Type = super::StreamsWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StreamsWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.configure_actions();
        }
    }

    impl WidgetImpl for StreamsWindow {}

    impl WindowImpl for StreamsWindow {
        fn close_request(&self) -> glib::Propagation {
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for StreamsWindow {}
    impl AdwApplicationWindowImpl for StreamsWindow {}

    impl StreamsWindow {
        fn configure_actions(&self) {
            self.sign_in_button.connect_clicked(|_arg| {
                println!("Clicked!");
                twitch::sign_in();
            });
        }
    }
}

glib::wrapper! {
    pub struct StreamsWindow(ObjectSubclass<imp::StreamsWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl StreamsWindow {
    pub fn new(app: &StreamsApplication) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
