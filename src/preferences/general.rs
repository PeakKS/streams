use gtk::{self, gio, glib};

mod imp {
    use adw::prelude::ActionRowExt;
    use adw::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, PreferencesPageImpl};
    use gtk::subclass::widget::{
        CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassExt, WidgetImpl,
    };
    use gtk::TemplateChild;

    use super::*;
    use crate::accounts::twitch;

    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/stream/streamsapp/Streams/ui/preferences/general.ui")]
    pub struct GeneralPreferences {
        #[template_child]
        pub twitch_account: TemplateChild<adw::ActionRow>,
    }

    impl Default for GeneralPreferences {
        fn default() -> Self {
            Self {
                twitch_account: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GeneralPreferences {
        const NAME: &'static str = "GeneralPreferences";
        type Type = super::GeneralPreferences;
        type ParentType = adw::PreferencesPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GeneralPreferences {
        fn constructed(&self) {
            self.parent_constructed();

            self.configure_actions();
        }
    }

    impl WidgetImpl for GeneralPreferences {}

    impl PreferencesPageImpl for GeneralPreferences {}

    impl GeneralPreferences {
        fn configure_actions(&self) {
            self.twitch_account.connect_activated(|_| {
                twitch::sign_in();
            });
        }
    }
}

glib::wrapper! {
    pub struct GeneralPreferences(ObjectSubclass<imp::GeneralPreferences>)
        @extends adw::PreferencesPage, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GeneralPreferences {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
