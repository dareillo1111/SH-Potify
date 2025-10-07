use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct Track {
        pub file_path: Option<String>,
        pub spotify_id: String,
        pub album_name: String,
        pub artist: String,
        pub name: String,
        #[sqlx(skip)]
        pub url: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct Playlist {
        pub spotify_id: String,
        pub name: String,
        #[sqlx(skip)]
        pub tracks: Option<Vec<Track>>,
}
