mod application;
mod window;
use application::StreamsApplication;
use gtk::{glib, gio};
fn main() {
    glib::set_application_name("Streams");

    let gresource_dir = if let Ok(gresource_dir) = std::env::var("STREAMS_RESOURCE_DIR") {
        gresource_dir
    } else {
        "/io/github/PeakKS/Streams/data/streams.gresource".to_owned()
    };

    let res = gio::Resource::load(&format!("{gresource_dir}/streams.gresource")).expect("Could not load gresources file");

    gio::resources_register(&res);
    let app = StreamsApplication::default();
    app.run();

}
