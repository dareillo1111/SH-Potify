use crate::{
        db, shpotify::{
                errors::ShpotifyErrors,
                music_entities::{Playlist, Track},
        }, spotify_api, track_downloader, AppState
};
use tokio::task::JoinSet;

pub mod errors;
pub mod music_entities;

pub async fn get_playlists(user_id: &String) -> Result<Vec<Playlist>, ShpotifyErrors> {
        spotify_api::get_playlists(user_id)
                .await
                .map_err(ShpotifyErrors::SpotifyAPIError)
}

//This function dosent care about download errors. It stores None if the download failed.
pub async fn download_playlists(playlists: Vec<Playlist>, state: AppState) -> () {
        let new_playlists = download(playlists).await;
        
        for playlist in new_playlists.iter(){
                db::insert_playlist(playlist, &state.db_pool);
        }
}

async fn download(mut playlists: Vec<Playlist>) -> Vec<Playlist> {
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
                let new_tracks_paths = join_set.join_all().await;
                let tracks = asign_tracks_paths(new_tracks_paths, playlist.tracks.clone());
                playlist.tracks = tracks;
        }
        playlists
}

fn asign_tracks_paths(
        new_tracks_paths: Vec<(String, Option<String>)>,
        mut tracks: Vec<Track>,
) -> Vec<Track> {
        for (id, path) in new_tracks_paths.into_iter() {
                for track in tracks.iter_mut() {
                        if track.spotify_id == id {
                                track.file_path = path.clone();
                        }
                }
        }
        return tracks;
}
