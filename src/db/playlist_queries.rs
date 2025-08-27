use crate::shpotify::music_entities::Playlist;

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

pub(crate) async fn select_all(
        connection: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<Vec<Playlist>, sqlx::Error> {
        let playlists: Vec<Playlist> =
                sqlx::query_as::<_, Playlist>("SELECT spotify_id, name FROM playlists")
                        .fetch_all(connection)
                        .await?;

        Ok(playlists)
}
