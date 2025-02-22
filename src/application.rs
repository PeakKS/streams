use adw::prelude::AdwDialogExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::window::StreamsWindow;
use crate::{config, preferences};

mod imp {
    use std::cell::OnceCell;

    use adw::subclass::prelude::{AdwApplicationImpl, ObjectSubclass};
    use gtk::glib::WeakRef;

    use super::*;
    use crate::window::StreamsWindow;

    #[derive(Debug, Default)]
    pub struct StreamsApplication {
        pub window: OnceCell<WeakRef<StreamsWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StreamsApplication {
        const NAME: &'static str = "StreamsApplication";
        type Type = super::StreamsApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for StreamsApplication {}

    impl ApplicationImpl for StreamsApplication {
        fn activate(&self) {
            self.parent_activate();
            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = StreamsWindow::new(&app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");
            app.main_window().present();
        }

        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();

            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for StreamsApplication {}
    impl AdwApplicationImpl for StreamsApplication {}
}

glib::wrapper! {
    pub struct StreamsApplication(ObjectSubclass<imp::StreamsApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl StreamsApplication {
    fn main_window(&self) -> StreamsWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                app.main_window().close();
                app.quit();
            })
            .build();

        let action_preferences = gio::ActionEntry::builder("preferences")
            .activate(move |app: &Self, _, _| {
                app.show_preferences();
            })
            .build();

        self.add_action_entries([action_quit, action_preferences]);
    }

    fn show_preferences(&self) {
        let preferences = preferences::PreferencesDialog::new();
        preferences.present(Some(&self.main_window()));
    }

    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    pub fn run(&self) -> glib::ExitCode {
        ApplicationExtManual::run(self)
    }
}

impl Default for StreamsApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", config::APP_ID)
            .build()
    }
}
