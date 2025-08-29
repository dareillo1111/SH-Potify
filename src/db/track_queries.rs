use crate::shpotify::music_entities::Track;

pub(crate) async fn insert(
        track: &crate::shpotify::music_entities::Track,
        connection: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO tracks (file_path, spotify_id, album_name, artist, name) VALUES (?,?,?,?,?)")
                .bind(&track.file_path)
                .bind(&track.spotify_id)
                .bind(&track.album_name)
                .bind(&track.artist)
                .bind(&track.name)
                .execute(connection)
                .await?;

        Ok(())
}

pub(crate) async fn select(
        connection: &sqlx::Pool<sqlx::Sqlite>,
        track_id: &str,
) -> Result<Track, sqlx::Error> {
        let track: Track =
                sqlx::query_as::<_, Track>("SELECT * FROM tracks WHERE spotify_id = (?)")
                        .bind(track_id)
                        .fetch_one(connection)
                        .await?;

        Ok(track)
}

pub(crate) async fn select_path(
        connection: &sqlx::Pool<sqlx::Sqlite>,
        track_id: &str,
) -> Result<String, sqlx::Error> {
        let path: String =
                sqlx::query_scalar("SELECT file_path FROM tracks WHERE spotify_id = (?)")
                        .bind(track_id)
                        .fetch_one(connection)
                        .await?;

        Ok(path)
}
