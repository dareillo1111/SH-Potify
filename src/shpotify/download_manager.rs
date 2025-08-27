use std::collections::HashMap;

use tokio::task::JoinSet;

use crate::{
        shpotify::music_entities::{Playlist, Track},
        track_downloader,
};

pub(crate) async fn download_playlist(mut playlists: Vec<Playlist>) -> Vec<Playlist> {
        for playlist in playlists.iter_mut() {
                let clone = playlist.clone();
                //I'm consuming the playlist variable in the for loop so i need to clone it.
                //There's a better way 100%
                let mut join_set = JoinSet::new();
                for track in clone.tracks.into_iter() {
                        join_set.spawn(async move {
                                let path = track_downloader::download_track(&track).await;
                                (track.spotify_id, path)
                        });
                }
                let download_paths = join_set.join_all().await;
                let paths: HashMap<String, Option<String>> = download_paths.into_iter().collect();

                let tracks = asign_tracks_paths(paths, playlist.tracks.clone());
                playlist.tracks = tracks;
        }
        playlists
}

fn asign_tracks_paths(
        mut download_paths: HashMap<String, Option<String>>,
        mut tracks: Vec<Track>,
) -> Vec<Track> {
        for track in tracks.iter_mut() {
                let id = &track.spotify_id;
                if let Some(path) = download_paths.remove(id) {
                        track.file_path = path;
                } else {
                        track.file_path = None;
                }
        }
        return tracks;
}
