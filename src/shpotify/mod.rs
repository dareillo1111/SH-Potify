use std::sync::Arc;

use axum_extra::headers::Range;
use sqlx::SqlitePool;
use tokio::sync::Semaphore;

use crate::{
        db,
        shpotify::{errors::ShpotifyErrors, music_entities::Playlist},
        spotify_api::{self, token_manager::TokenManager},
        track_streamer,
};

mod download_manager;
pub mod errors;
pub mod music_entities;

pub async fn get_playlists(
        user_id: &String,
        token_manager: &TokenManager,
) -> Result<Vec<Playlist>, ShpotifyErrors> {
        spotify_api::get_playlists(user_id, token_manager)
                .await
                .map_err(ShpotifyErrors::SpotifyAPIError)
}

pub async fn download_playlists(
        playlists_id: Vec<String>,
        db_pool: &SqlitePool,
        semaphore: Arc<Semaphore>,
        token_manager: &TokenManager,
) -> () {
        //This function dosent manage download errors. It stores None if the download failed.
        let playlists = spotify_api::get_playlists_by_id(playlists_id, token_manager).await;

        if let Ok(mut binding_playlist) = playlists {
                for playlist in binding_playlist.iter_mut() {
                        let playlist_clone = playlist.clone();
                        if let Some(tracks) = &mut playlist.tracks {
                                db::insert_playlist(&playlist_clone, db_pool).await;

                                for track in tracks.iter_mut() {
                                        let semaphore = semaphore.clone();
                                        let db_pool = db_pool.clone();
                                        let mut track = track.clone();
                                        let playlist = playlist_clone.clone();

                                        tokio::spawn(async move {
                                                let _permit = semaphore.acquire().await.unwrap();
                                                if let Some(_) =
                                                        download_manager::download_track(&mut track)
                                                                .await
                                                {
                                                        db::insert_playlist_track(
                                                                track, playlist, &db_pool,
                                                        )
                                                        .await;
                                                };
                                        });
                                }
                        }
                }
        }
}

pub async fn get_db_playlists(db_pool: &SqlitePool) -> Result<Vec<Playlist>, ShpotifyErrors> {
        let playlists = db::select_all_playlists(db_pool)
                .await
                .map_err(ShpotifyErrors::DbError)?;

        Ok(playlists)
}

pub(crate) async fn stream_track(
        track_id: String,
        range: Option<Range>,
        db_pool: &SqlitePool,
) -> Result<axum_range::Ranged<axum_range::KnownSize<tokio::fs::File>>, ShpotifyErrors> {
        let path = db::select_track_path(track_id, db_pool)
                .await
                .expect("failed");
        let range_stream = track_streamer::stream(&path, range).await;

        Ok(range_stream)
}
