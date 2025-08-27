use crate::{db::errors::DbErrors, shpotify::music_entities::Playlist};
use sqlx::{Pool, Sqlite, SqlitePool};

mod errors;
mod init_db;
mod playlist_queries;
mod playlist_track_queries;
mod track_queries;

const DB_URL: &str = "sqlite://temporal.db";

pub async fn init_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
        Ok(init_db::init_db(db_url).await?)
}

pub(crate) async fn insert_playlist(
        playlist: &Playlist,
        db_pool: &Pool<Sqlite>,
) -> Vec<Result<(), DbErrors>> {
        let mut errors: Vec<Result<(), DbErrors>> = Vec::new();

        let result = playlist_queries::insert(playlist, db_pool)
                .await
                .map_err(DbErrors::SqliteError);
        println!("inserted playlist? {:?}", result);
        if result.is_err() {
                errors.push(result);
        }

        for track in playlist.tracks.iter() {
                if track.file_path.is_none() {
                        errors.push(Err(DbErrors::TrackStructError(track.spotify_id.clone())));
                } else {
                        let result = track_queries::insert(track, db_pool)
                                .await
                                .map_err(DbErrors::SqliteError);
                        println!("inserted tracks? {:?}", result);
                        if result.is_err() {
                                errors.push(result);
                        }
                }
        }
        errors
}
