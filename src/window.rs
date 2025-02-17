use adw::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassIsExt};
use gtk::{gio::{self, prelude::SettingsExt}, glib, prelude::GtkWindowExt, subclass::widget::CompositeTemplateClass, TemplateChild};
use adw::subclass::prelude::{ObjectImplExt, ObjectSubclassExt};
use gtk::subclass::{prelude::ApplicationWindowImpl, widget::WidgetImpl, window::{WindowImpl, WindowImplExt}};
use adw::subclass::prelude::WidgetClassExt;

use crate::application::StreamsApplication;


mod imp {
    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use gtk::subclass::widget::CompositeTemplateInitializingExt;

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/PeakKS/Streams/gtk/window.ui")]
    pub struct StreamsWindow {
        #[template_child]
        pub split_view: TemplateChild<adw::OverlaySplitView>,

        #[template_child]
        pub stream_tab_bar: TemplateChild<adw::TabBar>,

        #[template_child]
        pub stream_tab_view: TemplateChild<adw::TabView>,
    }

    impl Default for StreamsWindow {
        fn default() -> Self {
            Self {
                split_view: TemplateChild::default(),
                stream_tab_bar: TemplateChild::default(),
                stream_tab_view: TemplateChild::default(),
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