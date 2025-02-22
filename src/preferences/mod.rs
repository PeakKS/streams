mod general;

use adw::prelude::PreferencesDialogExt;
use adw::subclass::dialog::AdwDialogImpl;
use adw::subclass::prelude::{ObjectImpl, ObjectSubclass, PreferencesDialogImpl};
use gtk::subclass::widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl};
use gtk::{self, gio, glib};

mod imp {
    use super::*;

    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/stream/streamsapp/Streams/ui/preferences/dialog.ui")]
    pub struct PreferencesDialog {}

    impl Default for PreferencesDialog {
        fn default() -> Self {
            Self {}
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesDialog {
        const NAME: &'static str = "PreferencesDialog";
        type Type = super::PreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PreferencesDialog {}

    impl WidgetImpl for PreferencesDialog {}

    impl AdwDialogImpl for PreferencesDialog {}

    impl PreferencesDialogImpl for PreferencesDialog {}
}

glib::wrapper! {
    pub struct PreferencesDialog(ObjectSubclass<imp::PreferencesDialog>)
        @extends adw::PreferencesDialog, adw::Dialog, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PreferencesDialog {
    pub fn new() -> Self {
        let dialog: Self = glib::Object::builder().build();
        dialog.add(&general::GeneralPreferences::new());
        dialog
    }
}
