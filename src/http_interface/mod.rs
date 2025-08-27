use crate::{
        shpotify::{self, music_entities::Playlist},
        AppState,
};
use axum::{
        extract::{Query, State},
        Json,
};
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
pub async fn select_db_playlists(
        State(state): State<AppState>,
) -> Result<Json<Vec<Playlist>>, HttpInterfaceErrors> {
        println!("hi, im shy uwu");
        let playlists = shpotify::get_db_playlists(state)
                .await
                .map_err(HttpInterfaceErrors::ShpotifyError)?;
        println!("called? {:?}", playlists);

        return Ok(Json(playlists));
}

#[axum::debug_handler]
pub async fn download_playlists(
        State(state): State<AppState>,
        Json(playlists): Json<Vec<Playlist>>,
) -> Result<StatusCode, HttpInterfaceErrors> {
        let _ = shpotify::download_playlists(playlists, state).await;
        println!("return?");
        Ok(StatusCode::OK)
}
