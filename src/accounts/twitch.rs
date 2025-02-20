use std::str::FromStr;

use twitch_api::twitch_oauth2::{self, url, ClientId, DeviceUserTokenBuilder, Scope};

use gtk::{
    gio::{ffi::GAsyncReadyCallback, Cancellable},
    glib::{self, clone},
};

use reqwest;

use crate::{config, runtime, window::StreamsWindow};

pub fn sign_in() {
    let client_id = ClientId::new(String::from(config::TWITCH_CLIENT_ID));
    let mut builder = DeviceUserTokenBuilder::new(client_id, vec![Scope::UserReadFollows]);

    let reqwest = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    runtime().spawn(async move {
        let code_response = match builder.start(&reqwest).await {
            Ok(code_response) => code_response,
            Err(error) => {
                eprintln!("{error}");
                return;
            }
        };

        let verification_uri = code_response.verification_uri.clone();
        glib::spawn_future(async move {
            gtk::UriLauncher::new(&verification_uri).launch(
                None::<&StreamsWindow>,
                Cancellable::NONE,
                |_| {
                    println!("Opened link");
                },
            );
        });

        let mut token = match builder.wait_for_code(&reqwest, tokio::time::sleep).await {
            Ok(token) => token,
            Err(error) => {
                eprintln!("{error}");
                return;
            }
        };

        println!("Got token: {token:?}");
    });
}
