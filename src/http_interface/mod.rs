use crate::{
        shpotify::{self, music_entities::Playlist},
        AppState,
};
use axum::{
        extract::{Path, Query, State},
        response::IntoResponse,
        Json,
};
use axum_extra::{headers::Range, TypedHeader};
use errors::*;
use reqwest::StatusCode;
use std::collections::HashMap;

mod errors;

#[axum::debug_handler]
pub async fn get_user_playlists(
        State(state): State<AppState>,
        Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Playlist>>, HttpInterfaceErrors> {
        if let Some(id) = params.get("user-id") {
                println!("serving");
                let token_manager = &state.token_manager;
                let playlists = shpotify::get_playlists(id, token_manager)
                        .await
                        .map_err(HttpInterfaceErrors::ShpotifyError)?;
                println!("correct");

                return Ok(Json(playlists));
        }

        Err(HttpInterfaceErrors::MissingQueryParam)
}

#[axum::debug_handler]
pub async fn select_db_playlists(
        State(state): State<AppState>,
) -> Result<Json<Vec<Playlist>>, HttpInterfaceErrors> {
        let playlists = shpotify::get_db_playlists(&state.db_pool)
                .await
                .map_err(HttpInterfaceErrors::ShpotifyError)?;

        return Ok(Json(playlists));
}

#[axum::debug_handler]
pub async fn download_playlists(
        State(state): State<AppState>,
        Json(playlists): Json<Vec<Playlist>>,
) -> Result<StatusCode, HttpInterfaceErrors> {
        let _ = shpotify::download_playlists(playlists, &state.db_pool, state.semaphore.clone())
                .await;

        Ok(StatusCode::OK)
}

#[axum::debug_handler]
pub async fn stream_track(
        State(state): State<AppState>,
        Path(track_id): Path<String>,
        range: Option<TypedHeader<Range>>,
) -> Result<impl IntoResponse, HttpInterfaceErrors> {
        let range = range.map(|TypedHeader(range)| range);
        let stream_track = shpotify::stream_track(track_id, range, &state.db_pool).await?;
        Ok(stream_track)
}
