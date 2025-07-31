use std::collections::HashMap;

use anyhow::{anyhow, Error};
use axum::{body::Body, extract::Query, http::Response, response::IntoResponse};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::{to_string, Value};
use crate::music_server::{cache_manager::cache_playlists, token_manager::{get_token, request_token, PotifyToken}};

#[derive(Debug, Serialize, Clone)]
pub struct Track {
        pub image_path: Option<String>,
        pub image_url: String,
        pub spotify_id: String,
        pub album_name: String,
        pub artist: String,
        pub name: String,
        pub url: String,
        pub duration_ms: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct PlayList {
        pub name: String,
        pub tracks: Vec<Track>,
        pub url: String,
}

async fn get_valid_token() -> Result<PotifyToken, Error> {
        if let None = get_token() {
                request_token().await?;
        }
        if let Some(token) = get_token() {
                Ok(token)
        } else {
                Err(anyhow!("Filed to get token"))
        }
}

pub async fn get_user_playlists(
        Query(user_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
        let user_id = user_params.get("user_id").expect("No user_id");

        let client = Client::new();

        let token = get_valid_token().await.unwrap();
        let auth_type = token.token_type;
        let auth_id = token.access_token;

        let result = client
                .get(format!(
                        "https://api.spotify.com/v1/users/{}/playlists",
                        user_id
                ))
                .header("Authorization", format!("{} {}", auth_type, auth_id))
                .send()
                .await;

        let response = match result {
                Ok(resp) => {
                        if resp.status() != StatusCode::OK {
                                return Response::builder()
                                        .status(resp.status())
                                        .body(Body::from(format!(
                                                "Spotify API responded with status: {}",
                                                resp.status()
                                        )))
                                        .unwrap();
                        }
                        resp
                }
                Err(err) => {
                        return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!(
                                        "Failed getting users playlists: {}",
                                        err
                                )))
                                .unwrap();
                }
        };

        let playlist_response = response.json::<Value>().await;

        let playlist_json = match playlist_response {
                Ok(play) => play,
                Err(err) => {
                        return Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(format!(
                                        "Failed parsing users playlists: {}",
                                        err
                                )))
                                .unwrap();
                }
        };

        let items = playlist_json
                .get("items")
                .and_then(|item| item.as_array())
                .unwrap();

        let mut playlists: Vec<PlayList> = Vec::new();
        for item in items.iter() {
                playlists.push(build_playlist(item).await);
        }

        let playlists_json = to_string(&playlists).expect("Failed serializing playlists");
        cache_playlists(playlists).await;

        Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(playlists_json))
                .unwrap()
}

async fn build_playlist(json_item: &Value) -> PlayList {
        let playlist_name = json_item
                .get("name")
                .and_then(|name| name.as_str())
                .unwrap()
                .to_string();

        let track_url = json_item
                .get("tracks")
                .and_then(|track| track.get("href"))
                .and_then(|href| href.as_str())
                .unwrap()
                .to_string();

        let playlist_url = json_item
                .get("external_urls")
                .and_then(|object| object.get("spotify"))
                .unwrap()
                .to_string();

        let tracks: Vec<Track> = request_tracks(track_url).await.unwrap();

        PlayList {
                name: playlist_name,
                tracks: tracks,
                url: playlist_url,
        }
}

async fn request_tracks(url: String) -> Result<Vec<Track>, reqwest::Error> {
        let token = get_valid_token().await.unwrap();

        let client = Client::new();
        let auth_type = token.token_type;
        let auth_id = token.access_token;

        let result = client
                .get(url)
                .header("Authorization", format!("{} {}", auth_type, auth_id))
                .send()
                .await?;

        let tracks_json = result.json::<Value>().await?;
        let items = tracks_json
                .get("items")
                .and_then(|item| item.as_array())
                .unwrap();

        let mut tracks: Vec<Track> = Vec::new();
        for item in items.iter() {
                let track_info = item.get("track").unwrap();

                let image_url: String = track_info
                        .get("album")
                        .and_then(|album| album.get("images"))
                        //Spoti API gives 3 images with diferent resolutions
                        //the second one is 300x300
                        .and_then(|images| images.get(1))
                        .and_then(|image| image.get("url"))
                        .and_then(|url| url.as_str())
                        .unwrap_or("No image")
                        .to_string();

                let spoti_id = track_info
                                .get("id")
                                .and_then(|id| id.as_str())
                                .unwrap_or("unknown")
                                .to_string();

                let track: Track = Track {
                        image_path: None,
                        image_url: image_url,
                        spotify_id: spoti_id,
                        album_name: track_info
                                .get("album")
                                .and_then(|album| album.get("name"))
                                .and_then(|name| name.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                        artist: track_info
                                .get("artists")
                                .and_then(|artists| artists.get(0))
                                .and_then(|artist| artist.get("name"))
                                .and_then(|name| name.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                        url: track_info
                                .get("external_urls")
                                .and_then(|url| url.get("spotify"))
                                .and_then(|url| url.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                        name: track_info
                                .get("name")
                                .and_then(|name| name.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                        duration_ms: track_info
                                .get("duration_ms")
                                .and_then(|duration| duration.as_u64())
                                .unwrap_or(0),
                };
                tracks.push(track);
        }

        Ok(tracks)
}
