use crate::spotify_api::errors::SpotifyAPIErrors as SpotiErr;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

//move to .env
const CLIENT_ID: &str = "07d4774515e1425e831920b95399d372";
const CLIENT_SECRET: &str = "1337642391704d18b974793341092d9e";

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Token {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: u64,
}

#[derive(Clone)]
pub(crate) struct TokenManager {
        token: Arc<tokio::sync::RwLock<Token>>,
        is_expired: Arc<tokio::sync::RwLock<bool>>,
}

impl TokenManager {
        pub fn new() -> Self {
                Self {
                        token: Arc::new(RwLock::new(Token {
                                access_token: "null".to_string(),
                                token_type: "null".to_string(),
                                expires_in: 0,
                        })),
                        is_expired: Arc::new(RwLock::new(true)),
                }
        }

        pub async fn get_token(&self) -> Result<Token, SpotiErr> {
                print!("Asked token");
                let is_expired = Arc::clone(&self.is_expired);
                let token = Arc::clone(&self.token);
                let read_guard = is_expired.read().await;
                if *read_guard == true {
                        println!("Token expired, asking for a new one");
                        drop(read_guard);
                        TokenManager::request_token(token, is_expired).await?;
                }

                let token = Arc::clone(&self.token);
                let token_read_guard = token.read().await;
                println!("Returning token");
                Ok(token_read_guard.clone())
        }

        async fn request_token(
                token: Arc<tokio::sync::RwLock<Token>>,
                is_expired: Arc<tokio::sync::RwLock<bool>>,
        ) -> Result<(), SpotiErr> {
                let mut form = HashMap::new();
                form.insert("grant_type", "client_credentials");

                let encoded_auth =
                        general_purpose::STANDARD.encode(format!("{CLIENT_ID}:{CLIENT_SECRET}"));

                let token_request = Client::new()
                        .post("https://accounts.spotify.com/api/token")
                        .header("Authorization", format!("Basic {encoded_auth}"))
                        .form(&form);

                let response = token_request
                        .send()
                        .await
                        .map_err(SpotiErr::RequestFailed)?;

                let token_response = response
                        .json::<Token>()
                        .await
                        .map_err(SpotiErr::JsonParsingFailed)?;

                TokenManager::set_token_timeout(&token_response.expires_in, is_expired).await;
                TokenManager::set_token(token, token_response).await;

                Ok(())
        }

        async fn set_token_timeout(
                expires_in: &u64,
                is_expired: Arc<tokio::sync::RwLock<bool>>,
        ) -> () {
                let expires_in = expires_in.to_owned();
                let is_expired_pre_thread = Arc::clone(&is_expired);

                let mut write_guard = is_expired_pre_thread.write().await;
                *write_guard = false;
                println!("Assign token timeout");

                tokio::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_secs(expires_in)).await;
                        let mut write_guard = is_expired.write().await;
                        *write_guard = true;
                });
        }

        async fn set_token(token: Arc<tokio::sync::RwLock<Token>>, token_response: Token) -> () {
                println!("Cahing token");
                let mut write_guard = token.write().await;
                *write_guard = token_response;
        }
}
