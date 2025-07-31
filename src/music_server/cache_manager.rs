use crate::music_server::spoty_queries::PlayList;
use moka::future::Cache;
use once_cell::sync::Lazy;
use std::time::Duration;

static CACHED_TRACKS: Lazy<Cache<String, PlayList>> = Lazy::new(|| {
        Cache::builder()
                .time_to_live(Duration::from_secs(60))
                .build()
});

pub async fn cache_playlists(playlists: Vec<PlayList>) {
        for playlist in playlists.iter() {
                let url = playlist.url.clone();
                CACHED_TRACKS.insert(url, playlist.clone()).await;
        }
}

pub async fn get_cached_playlist(playlist_url: &str) -> Option<PlayList> {
        CACHED_TRACKS.get(playlist_url).await
}
