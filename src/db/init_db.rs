use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};

pub(crate) async fn init_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
                Sqlite::create_database(db_url).await?;
                println!("Crated db");
        } else {
                println!("Database already exists");
        }
        let connection = SqlitePool::connect(db_url).await?;
        create_tables(&connection).await?;

        let result = sqlx::query(
                "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';",
        )
        .fetch_all(&connection)
        .await
        .unwrap();

        for (idx, row) in result.iter().enumerate() {
                println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
        }
        Ok(connection)
}

async fn create_tables(connection: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query("CREATE TABLE IF NOT EXISTS playlists (spotify_id VARCHAR(250) PRIMARY KEY NOT NULL, name VARCHAR(250))").execute(connection).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS tracks (file_path VARCHAR(250), image_path VARCHAR(250), spotify_id VARCHAR(250) PRIMARY KEY NOT NULL, album_name VARCHAR(250), artist VARCHAR(250), name VARCHAR(250))").execute(connection).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS playlist_track (track_id VARCHAR(250) NOT NULL, playlist_id VARCHAR(250) NOT NULL, PRIMARY KEY (track_id, playlist_id), FOREIGN KEY (track_id) REFERENCES tracks(spotify_id), FOREIGN KEY (playlist_id) REFERENCES playlists(spotify_id))").execute(connection).await?;

        Ok(())
}
