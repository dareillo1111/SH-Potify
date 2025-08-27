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
        println!("{:?}", &track.file_path);
        Ok(())
}

pub(crate) async fn select(
        connection: &sqlx::Pool<sqlx::Sqlite>,
        track_id: &str,
) -> Result<Track, sqlx::Error> {
        println!("someone asked me");
        let track: Track =
                sqlx::query_as::<_, Track>("SELECT * FROM tracks WHERE spotify_id = (?)")
                        .bind(track_id)
                        .fetch_one(connection)
                        .await?;
        println!("{:?}", track);
        Ok(track)
}
