use crate::{db::errors::DbErrors, shpotify::music_entities::Playlist};
use sqlx::{Pool, Sqlite, SqlitePool};

pub mod errors;
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

                        if result.is_err() {
                                errors.push(result);
                        } else {
                                playlist_track_queries::insert(
                                        &track.spotify_id,
                                        &playlist.spotify_id,
                                        db_pool,
                                )
                                .await;
                        }
                }
        }
        errors
}

pub(crate) async fn select_all_playlists(
        db_pool: &Pool<Sqlite>,
) -> Result<Vec<Playlist>, DbErrors> {
        println!("in3");
        let mut playlists = playlist_queries::select_all(db_pool)
                .await
                .map_err(DbErrors::SqliteError)?;
        println!("consulta playlists: {:?}", playlists);

        println!("in3");
        for playlist in playlists.iter_mut() {
                let playlist_tracks_id: Vec<String> =
                        playlist_track_queries::select_tracks_id(db_pool, &playlist.spotify_id)
                                .await
                                .map_err(DbErrors::SqliteError)?;
                println!("consulta tracks_id: {:?}", playlist_tracks_id);

                println!("in3");
                let mut tracks = Vec::new();
                for track_id in playlist_tracks_id.iter() {
                        println!("inone1?");
                        let track = track_queries::select(db_pool, track_id)
                                .await
                                .map_err(DbErrors::SqliteError)?;
                        println!("{:?}", track);
                        tracks.push(track);
                }
                println!("in2");
                playlist.tracks = tracks;
        }

        Ok(playlists)
}
