use crate::{shpotify::{self, music_entities::Playlist}, AppState};
use axum::{extract::{Query, State}, Json};
use errors::*;
use reqwest::StatusCode;
use std::collections::HashMap;

mod errors;

#[axum::debug_handler]
pub async fn get_user_playlists(
        Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Playlist>>, HttpInterfaceErrors> {
        if let Some(id) = params.get("user-id") {
                let playlists = shpotify::get_playlists(id)
                        .await
                        .map_err(HttpInterfaceErrors::ShpotifyError)?;

                return Ok(Json(playlists));
        }
        Err(HttpInterfaceErrors::MissingQueryParam)
}

#[axum::debug_handler]
pub async fn download_playlists(
        State(state): State<AppState>,
        Json(playlists): Json<Vec<Playlist>>,
) -> Result<StatusCode, HttpInterfaceErrors> {
        /*let mut valid_playlists = Vec::new();
                for playlist in playlists.into_iter() {
                        if !playlist.tracks.is_empty() {
                                println!("Empty?: \n{:?}", playlist.tracks);
                                valid_playlists.push(playlist);
                        }
                }
                if valid_playlists.is_empty() {
                        return Err(HttpInterfaceErrors::EmptyPlaylist);
                }

        */
        let _ = shpotify::download_playlists(playlists, state).await;
        println!("return?");
        Ok(StatusCode::OK)
}
