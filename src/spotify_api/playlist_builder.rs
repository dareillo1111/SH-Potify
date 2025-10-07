use std::collections::HashMap;

use crate::{
        shpotify::music_entities::{Playlist, Track},
        spotify_api::errors::SpotifyAPIErrors as SpotiErr,
};

pub(crate) fn get_playlist_tracks_href(
        json_playlist: &serde_json::Value,
) -> Option<HashMap<String, String>> {
        let mut tracks_href = HashMap::new();

        let track_url = json_playlist
                .get("tracks")
                .and_then(|track| track.get("href"))
                .and_then(|href| href.as_str())?
                .to_string();

        let playlist_id = json_playlist
                .get("id")
                .and_then(|id| id.as_str())?
                .to_string();

        tracks_href.insert(playlist_id, track_url);
        Some(tracks_href)
}

pub(crate) fn build_tracks(json_track: &serde_json::Value) -> Result<Vec<Track>, SpotiErr> {
        let items = json_track
                .get("items")
                .and_then(|item| item.as_array())
                .ok_or(SpotiErr::EmptyPlaylist)?;
        let mut tracks: Vec<Track> = Vec::new();

        for item in items.iter() {
                let track_info = item.get("track").ok_or(SpotiErr::EmptyPlaylist)?;

                let spotify_id = track_info
                        .get("id")
                        .and_then(|id| id.as_str())
                        .unwrap_or("Failed getting id")
                        .to_string();

                let album_name = track_info
                        .get("album")
                        .and_then(|album| album.get("name"))
                        .and_then(|name| name.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                let artist = track_info
                        .get("artists")
                        .and_then(|artists| artists.get(0))
                        .and_then(|artist| artist.get("name"))
                        .and_then(|name| name.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                let url = track_info
                        .get("external_urls")
                        .and_then(|url| url.get("spotify"))
                        .and_then(|url| url.as_str())
                        .unwrap_or("failed getting url")
                        .to_string();

                let name = track_info
                        .get("name")
                        .and_then(|name| name.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                let track: Track = Track {
                        file_path: None,
                        spotify_id,
                        album_name,
                        artist,
                        url,
                        name,
                };
                tracks.push(track);
        }
        Ok(tracks)
}

pub(crate) fn build_playlists(
        tracks: &mut Option<HashMap<String, Vec<Track>>>,
        json_playlists: serde_json::Value,
) -> Result<Vec<Playlist>, SpotiErr> {
        let items = json_playlists
                .get("items")
                .and_then(|item| item.as_array())
                .ok_or(SpotiErr::NoPlaylists)?;

        let mut playlists: Vec<Playlist> = Vec::new();

        for item in items.iter() {
                playlists.push(build_playlist(tracks, item)?);
        }

        Ok(playlists)
}

pub(crate) fn build_playlist(
        tracks: &mut Option<HashMap<String, Vec<Track>>>,
        json_playlist: &serde_json::Value,
) -> Result<Playlist, SpotiErr> {
        if let Some(spotify_id) = json_playlist.get("id").and_then(|id| id.as_str()) {
                let name = json_playlist
                        .get("name")
                        .and_then(|name| name.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                if let Some(tracks_map) = tracks.as_mut() {
                        //reminder: remove returns the value of the key before deleting it
                        if let Some(tracks) = tracks_map.remove(spotify_id) {
                                return Ok(Playlist {
                                        spotify_id: spotify_id.to_string(),
                                        name,
                                        tracks: Some(tracks),
                                });
                        };
                } else {
                        return Ok(Playlist {
                                spotify_id: spotify_id.to_string(),
                                name,
                                tracks: None,
                        });
                };
        }
        Err(SpotiErr::NoPlaylists)
}
