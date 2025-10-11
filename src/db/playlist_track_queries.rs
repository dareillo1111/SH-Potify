pub(crate) async fn insert(
        track_id: &str,
        playlist_id: &str,
        connection: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO playlist_track (playlist_id, track_id) VALUES (?, ?)")
                .bind(playlist_id)
                .bind(track_id)
                .execute(connection)
                .await?;
        Ok(())
}

pub(crate) async fn select_tracks_id(
        connection: &sqlx::Pool<sqlx::Sqlite>,
        playlist_id: &str,
) -> Result<Vec<String>, sqlx::Error> {
        let playlists: Vec<String> =
        //Returns the value of the first column, without a key.
                sqlx::query_scalar("SELECT track_id FROM playlist_track WHERE playlist_id = (?)")
                        .bind(playlist_id)
                        .fetch_all(connection)
                        .await?;
        Ok(playlists)
}
