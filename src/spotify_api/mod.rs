use crate::shpotify::music_entities::{Playlist, Track};
use crate::spotify_api::errors::SpotifyAPIErrors as SpotiErr;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use token_manager::TokenManager;

pub mod errors;
mod playlist_builder;
pub mod token_manager;

/* Maybe poor design. This mod and playlist_builder play ping pong. They call each other multiple times, this could be avoided by having one of those mods be responsable of 2 functions: requesting to the api, and processing the result. I wanted them to have only 1 function this is why is a bit uneficient. */
pub async fn get_playlists(
        user_id: &String,
        token_manager: &TokenManager,
) -> Result<Vec<Playlist>, SpotiErr> {
        let json_playlist = get_playlist_base(user_id, token_manager).await?;
        let playlists = playlist_builder::build_playlists(&mut None, json_playlist)?;

        Ok(playlists)
}

async fn get_playlist_base(
        user_id: &String,
        token_manager: &TokenManager,
) -> Result<Value, SpotiErr> {
        let token = token_manager.get_token().await?;

        let playlists_request = Client::new()
                .get(format!(
                        "https://api.spotify.com/v1/users/{user_id}/playlists"
                ))
                .header(
                        "Authorization",
                        format!("{} {}", token.token_type, token.access_token),
                )
                .send()
                .await?;

        if playlists_request.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(SpotiErr::UserIdNotFound);
        }

        let json_response = playlists_request.json::<Value>().await?;

        Ok(json_response)
}

async fn get_tracks(
        tracks_href: HashMap<String, String>,
        token_manager: &TokenManager,
) -> Result<HashMap<String, Vec<Track>>, SpotiErr> {
        let token = token_manager.get_token().await?;
        let mut playlist_tracks: HashMap<String, Vec<Track>> = HashMap::new();

        /* playlist_id is the key of the HashMap, href the value. We are iterating both at the same time  */
        for (playlist_id, href) in tracks_href.iter() {
                println!("Requesting track");
                let track_request = Client::new()
                        .get(href)
                        .header(
                                "Authorization",
                                format!("{} {}", token.token_type, token.access_token),
                        )
                        .send()
                        .await?;
                println!("Recived track");

                let json_track = track_request.json::<Value>().await?;

                let track = playlist_builder::build_tracks(&json_track)?;
                playlist_tracks.insert(playlist_id.to_string(), track);
                println!("Builded track");
        }
        Ok(playlist_tracks)
}

pub(crate) async fn get_playlists_by_id(
        playlists_ids: Vec<String>,
        token_manager: &TokenManager,
) -> Result<Vec<Playlist>, SpotiErr> {
        let mut playlists: Vec<Playlist> = Vec::new();

        for playlist_id in playlists_ids.iter() {
                let json_playlist = request_playlist_by_id(playlist_id, token_manager).await?;
                println!("new playlist: {:#?}", json_playlist);

                let tracks_href: HashMap<String, String> =
                        playlist_builder::get_playlist_tracks_href(&json_playlist)
                                .ok_or(SpotiErr::EmptyPlaylist)?;

                let tracks: HashMap<String, Vec<Track>> =
                        get_tracks(tracks_href, token_manager).await?;
                let playlist = playlist_builder::build_playlist(&mut Some(tracks), &json_playlist)?;
                playlists.push(playlist);
        }

        Ok(playlists)
}

async fn request_playlist_by_id(
        id: &String,
        token_manager: &TokenManager,
) -> Result<Value, SpotiErr> {
        let token = token_manager.get_token().await?;

        let playlists_request = Client::new()
                .get(format!("https://api.spotify.com/v1/playlists/{id}"))
                .header(
                        "Authorization",
                        format!("{} {}", token.token_type, token.access_token),
                )
                .send()
                .await?;

        if playlists_request.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(SpotiErr::UserIdNotFound);
        }

        let json_playlist = playlists_request.json::<Value>().await?;

        Ok(json_playlist)
}
