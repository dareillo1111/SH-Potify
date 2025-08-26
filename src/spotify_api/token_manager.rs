use crate::spotify_api::errors::SpotifyAPIErrors as SpotiErr;
use base64::{engine::general_purpose, Engine as _};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const CLIENT_ID: &str = "07d4774515e1425e831920b95399d372";
const CLIENT_SECRET: &str = "1337642391704d18b974793341092d9e";
static TOKEN: Lazy<tokio::sync::Mutex<Option<Token>>> = Lazy::new(|| tokio::sync::Mutex::new(None));

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Token {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i32,
}

pub(crate) async fn get_token() -> Result<Token, SpotiErr> {
        let mut guard = TOKEN.lock().await;
        if guard.is_none() {
                let new_token = request_token().await?;
                *guard = Some(new_token);
        }

        guard.clone().ok_or(SpotiErr::CachingTokenFailed)
}

async fn request_token() -> Result<Token, SpotiErr> {
        let mut form = HashMap::new();
        form.insert("grant_type", "client_credentials");

        let encoded_auth = general_purpose::STANDARD.encode(format!("{CLIENT_ID}:{CLIENT_SECRET}"));

        let token_request = Client::new()
                .post("https://accounts.spotify.com/api/token")
                .header("Authorization", format!("Basic {encoded_auth}"))
                .form(&form);

        let response = token_request
                .send()
                .await
                .map_err(SpotiErr::RequestFailed)?;

        let token = response
                .json::<Token>()
                .await
                .map_err(SpotiErr::JsonParsingFailed)?;

        set_token_timeout(token.expires_in);

        Ok(token)
}

fn set_token_timeout(expires_in: i32) {
        tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(expires_in as u64)).await;
                let mut guard = TOKEN.lock().await;
                *guard = None;
        });
}
