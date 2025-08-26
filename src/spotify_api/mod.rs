use crate::shpotify::music_entities::{Playlist, Track};
use crate::spotify_api::errors::SpotifyAPIErrors as SpotiErr;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub mod errors;
mod playlist_builder;
mod token_manager;

/* Maybe poor design. This mod and playlist_builder play ping pong. They call each other multiple times, this could be avoided by having one of those mods be responsable of 2 functions: requesting to the api, and processing the result. I wanted them to have only 1 function this is why is a bit uneficient. */

pub async fn get_playlists(user_id: &String) -> Result<Vec<Playlist>, SpotiErr> {
        let token = token_manager::get_token().await?;

        let playlists_request = Client::new()
                .get(format!(
                        "https://api.spotify.com/v1/users/{user_id}/playlists"
                ))
                .header(
                        "Authorization",
                        format!("{} {}", token.token_type, token.access_token),
                )
                .send()
                .await
                .map_err(SpotiErr::RequestFailed)?;

        if playlists_request.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(SpotiErr::UserIdNotFound);
        }

        let json_response = playlists_request
                .json::<Value>()
                .await
                .map_err(SpotiErr::JsonParsingFailed)?;

        let tracks_href: HashMap<String, String> =
                playlist_builder::get_playlists_tracks_href(&json_response)
                        .ok_or(SpotiErr::EmptyPlaylist)?;

        let tracks: HashMap<String, Vec<Track>> = get_tracks(tracks_href).await?;
        let playlist = playlist_builder::build_playlist(tracks, json_response)?;

        Ok(playlist)
}

async fn get_tracks(
        tracks_href: HashMap<String, String>,
) -> Result<HashMap<String, Vec<Track>>, SpotiErr> {
        let token = token_manager::get_token().await?;
        let mut playlist_tracks: HashMap<String, Vec<Track>> = HashMap::new();

        /* playlist_id is the key of the HashMap, href the value. We are iterating both at the          same time  */
        for (playlist_id, href) in tracks_href.iter() {
                let track_request = Client::new()
                        .get(href)
                        .header(
                                "Authorization",
                                format!("{} {}", token.token_type, token.access_token),
                        )
                        .send()
                        .await
                        .map_err(SpotiErr::RequestFailed)?;

                let json_track = track_request
                        .json::<Value>()
                        .await
                        .map_err(SpotiErr::JsonParsingFailed)?;

                let track = playlist_builder::build_track(&json_track)?;
                playlist_tracks.insert(playlist_id.to_string(), track);
        }
        Ok(playlist_tracks)
}
