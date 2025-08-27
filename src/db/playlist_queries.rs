pub(crate) async fn insert(
        playlist: &crate::shpotify::music_entities::Playlist,
        connection: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO playlists (spotify_id, name) VALUES (?, ?)")
                .bind(&playlist.spotify_id)
                .bind(&playlist.name)
                .execute(connection)
                .await?;
        Ok(())
}
