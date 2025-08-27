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
        println!("{:?}", &track.file_path);
        Ok(())
}
