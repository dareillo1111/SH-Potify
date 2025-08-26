use crate::shpotify::music_entities::Playlist;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

mod init_db;

const DB_URL: &str = "sqlite://temporal.db";

pub async fn init_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
        Ok(init_db::init_db(db_url).await?)
}

pub(crate) fn insert_playlist(playlist: &Playlist, db_pool: &Pool<Sqlite>) -> () {
        ()
}


