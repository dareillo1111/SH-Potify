use axum_extra::headers::Range;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::{
        db,
        shpotify::{
                errors::ShpotifyErrors,
                music_entities::{Playlist, Track},
        },
        spotify_api::{self, token_manager::TokenManager},
        track_downloader, track_streamer,
};

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
) -> Result<(), ShpotifyErrors> {
        let playlists = spotify_api::get_playlists_by_id(playlists_id, token_manager).await?;

        for playlist in playlists.iter() {
                if let Some(tracks) = &playlist.tracks {
                        db::insert_playlist(&playlist, db_pool).await?;
                        for track in tracks.iter() {
                                download_thread(
                                        semaphore.clone(),
                                        track.clone(),
                                        playlist.spotify_id.clone(),
                                        db_pool.clone(),
                                );
                        }
                }
        }
        Ok(())
}

fn download_thread(
        semaphore: Arc<Semaphore>,
        mut track: Track,
        playlist_id: String,
        db_pool: SqlitePool,
) -> () {
        tokio::spawn(async move {
                // we lose the errors here, too advanced for me right now. JoinHandle
                let _permit = semaphore.acquire().await.unwrap();
                if let Some(path) = track_downloader::download_track(&mut track).await {
                        track.file_path = Some(path);
                        let _ = db::insert_playlist_track(track, &playlist_id, &db_pool).await;
                };
        });
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
        println!("Full track path {path}");
        let range_stream = track_streamer::stream(&path, range).await;

        Ok(range_stream)
}
