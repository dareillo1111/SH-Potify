use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
        pub file_path: Option<String>,
        pub image_path: Option<String>,
        pub spotify_id: String,
        pub album_name: String,
        pub artist: String,
        pub name: String,
        pub image_url: Option<String>,
        pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playlist {
        pub spotify_id: String,
        pub name: String,
        pub tracks: Vec<Track>,
}
