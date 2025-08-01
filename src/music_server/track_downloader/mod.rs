use axum::Json;
use serde::Deserialize;

use crate::music_server::{
        cache_manager::get_cached_playlist, spoty_queries::PlayList,
        track_downloader::track_image_downloader::download_track_image,
};

pub mod track_image_downloader;

#[derive(Deserialize, Debug)]
pub struct DownloadPlaylistParams {
        playlists_url: Vec<String>,
}

pub async fn download_playlists(Json(body): Json<DownloadPlaylistParams>) -> () {
        let playlists_url = body.playlists_url;
        println!("{:?}", playlists_url);

        for url in playlists_url.iter() {
                if let Some(playlist) = get_cached_playlist(url).await {
                        for track in playlist.tracks.iter() {
                                download_track_image(&track.image_url, &track.spotify_id).await.unwrap();
                        }
                } else{
                        println!("no playlists found");
                }
        }
}
