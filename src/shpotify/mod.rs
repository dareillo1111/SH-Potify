use crate::{
        db,
        shpotify::{errors::ShpotifyErrors, music_entities::Playlist},
        spotify_api, AppState,
};

mod download_manager;
pub mod errors;
pub mod music_entities;

pub async fn get_playlists(user_id: &String) -> Result<Vec<Playlist>, ShpotifyErrors> {
        spotify_api::get_playlists(user_id)
                .await
                .map_err(ShpotifyErrors::SpotifyAPIError)
}

//This function dosent care about download errors. It stores None if the download failed.
pub async fn download_playlists(playlists: Vec<Playlist>, state: AppState) -> () {
        let new_playlists = download_manager::download_playlist(playlists).await;

        for playlist in new_playlists.iter() {
                db::insert_playlist(playlist, &state.db_pool).await;
        }
}
