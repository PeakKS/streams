mod accounts;
mod application;
mod window;

mod config {
    include!(concat!(env!("BUILD_ROOT"), "/src/config.rs"));
}

use std::sync::OnceLock;

use application::StreamsApplication;
use gtk::{gio, glib};
use tokio::runtime::Runtime;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to initialize tokio runtime"))
}

fn main() {
    glib::set_application_name("Streams");

    let gresource_dir = if let Ok(gresource_dir) = std::env::var("STREAMS_RESOURCE_DIR") {
        gresource_dir
    } else {
        // Match Gio Application behavior of creating resource base path from app id
        format!("/{}/resources", config::APP_ID.replace(".", "/"))
    };

    let res = gio::Resource::load(&format!("{gresource_dir}/streams.gresource"))
        .expect("Could not load gresources file");

    gio::resources_register(&res);
    let app = StreamsApplication::default();
    app.run();
}
