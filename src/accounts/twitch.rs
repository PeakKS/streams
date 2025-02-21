use std::string::FromUtf8Error;

use gtk::gio::Cancellable;
use gtk::glib;
use keyring::Entry;
use reqwest;
use twitch_api::twitch_oauth2::tokens::errors::{DeviceUserTokenExchangeError, RefreshTokenError};
use twitch_api::twitch_oauth2::{
    AccessToken, ClientId, DeviceUserTokenBuilder, RefreshToken, Scope,
};

use crate::window::StreamsWindow;
use crate::{config, runtime};

const KEYRING_SERVICE: &str = "streams";
const KEYRING_USER: &str = "twitch-refresh";

const TWITCH_SCOPES: &[Scope] = &[Scope::UserReadFollows];

#[allow(dead_code)] // Don't really care about handling these, but want them for debugging information
#[derive(Debug)]
enum TokenError {
    OAuthError(DeviceUserTokenExchangeError<reqwest::Error>),
    TokenParseError(FromUtf8Error),
    RefreshError(RefreshTokenError<reqwest::Error>),
    KeyringError(keyring::Error),
}

impl From<DeviceUserTokenExchangeError<reqwest::Error>> for TokenError {
    fn from(value: DeviceUserTokenExchangeError<reqwest::Error>) -> Self {
        Self::OAuthError(value)
    }
}

impl From<FromUtf8Error> for TokenError {
    fn from(value: FromUtf8Error) -> Self {
        Self::TokenParseError(value)
    }
}

impl From<RefreshTokenError<reqwest::Error>> for TokenError {
    fn from(value: RefreshTokenError<reqwest::Error>) -> Self {
        Self::RefreshError(value)
    }
}

impl From<keyring::Error> for TokenError {
    fn from(value: keyring::Error) -> Self {
        Self::KeyringError(value)
    }
}

pub fn sign_in() {
    runtime().spawn(async move {
        println!("Refreshing Twitch access token...");
        let _access_token = match match refresh_token().await {
            Ok(access_token) => Ok(access_token),
            Err(error) => {
                eprintln!("Failed to refresh token: {error:?}");
                println!("Getting new access token...");
                get_new_token().await
            }
        } {
            Ok(access_token) => access_token,
            Err(error) => {
                eprintln!("Failed to get new token: {error:?}");
                return;
            }
        };

        println!("Got access token successfully");
    });
}

async fn refresh_token() -> Result<AccessToken, TokenError> {
    let client_id = ClientId::new(String::from(config::TWITCH_CLIENT_ID));
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).unwrap();
    let saved_refresh_token = entry.get_secret()?;
    let saved_refresh_token = String::from_utf8(saved_refresh_token)?;
    let refresh_token = RefreshToken::from(saved_refresh_token);

    let reqwest = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // TODO: Schedule a task that keeps access_token refreshed before duration expires
    let (access_token, _duration, new_refresh_token) = refresh_token
        .refresh_token(&reqwest, &client_id, None)
        .await?;

    if let Some(new_refresh_token) = new_refresh_token {
        entry.set_secret(new_refresh_token.secret().as_bytes())?;
    } else {
        eprintln!("Did not receive a new refresh token after refreshing");
    }

    Ok(access_token)
}

async fn get_new_token() -> Result<AccessToken, TokenError> {
    let client_id = ClientId::new(String::from(config::TWITCH_CLIENT_ID));
    let mut builder = DeviceUserTokenBuilder::new(client_id, TWITCH_SCOPES.to_vec());

    let reqwest = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let code_response = builder.start(&reqwest).await?;

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

    let token = builder.wait_for_code(&reqwest, tokio::time::sleep).await?;

    if let Some(refresh_token) = token.refresh_token {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).unwrap();
        entry.set_secret(refresh_token.secret().as_bytes())?;
    }

    Ok(token.access_token)
}
