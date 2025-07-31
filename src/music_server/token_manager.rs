use base64::{engine::general_purpose, Engine};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::{
        collections::HashMap,
        env,
        sync::Mutex,
        thread::{self, sleep},
        time::Duration,
};

#[derive(Debug, Deserialize, Clone)]
pub struct PotifyToken {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: u64,
}

static TOKEN: Lazy<Mutex<Option<PotifyToken>>> = Lazy::new(|| {
        Mutex::new(None)
});

pub async fn request_token() -> Result<(), reqwest::Error> {
        dotenv::dotenv().ok();
        let client_id = env::var("SPOTIFY_CLIENT_ID").expect("No client id");
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("No secret");
        let encoded_auth = general_purpose::STANDARD.encode(format!("{client_id}:{client_secret}"));

        let client = Client::new();

        let mut form = HashMap::new();
        form.insert("grant_type", "client_credentials");

        let result = client
                .post("https://accounts.spotify.com/api/token")
                .header("Authorization", format!("Basic {encoded_auth}"))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .form(&form)
                .send()
                .await?;

        let json_token = result.json::<PotifyToken>().await?;
        let token_timeout = json_token.expires_in;
        set_token_timeout(token_timeout);

        let mut guard = TOKEN.lock().unwrap();
        *guard = Some(json_token);

        Ok(())
}

pub fn get_token() -> Option<PotifyToken> {
        let guard = TOKEN.lock().unwrap();
        guard.clone()
}

fn set_token_timeout(timeout: u64) {
        thread::spawn(move || {
                sleep(Duration::from_secs(timeout));
                let mut guard = TOKEN.lock().unwrap();
                *guard = None;
        });
}
