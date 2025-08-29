use std::sync::Arc;

use axum_extra::{headers::Range, TypedHeader};
use sqlx::SqlitePool;
use tokio::sync::Semaphore;

use crate::{
        db,
        shpotify::{errors::ShpotifyErrors, music_entities::Playlist},
        spotify_api, track_streamer, AppState,
};

mod download_manager;
pub mod errors;
pub mod music_entities;

pub async fn get_playlists(user_id: &String) -> Result<Vec<Playlist>, ShpotifyErrors> {
        spotify_api::get_playlists(user_id)
                .await
                .map_err(ShpotifyErrors::SpotifyAPIError)
}

pub async fn download_playlists(playlists: Vec<Playlist>, db_pool: &SqlitePool, semaphore: Arc<Semaphore>) -> () {
        //This function dosent care about download errors. It stores None if the download failed.
        let new_playlists = download_manager::download_playlist(playlists, semaphore).await;
        println!("downloaded_playlists: {:?}", new_playlists);

        for playlist in new_playlists.iter() {
                println!("inserting: {:?}", playlist);
                db::insert_playlist(playlist, db_pool).await;
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
