use std::collections::HashMap;

use crate::{
        shpotify::music_entities::{Playlist, Track},
        spotify_api::errors::SpotifyAPIErrors as SpotiErr,
};

pub(crate) fn get_playlists_tracks_href(
        json_response: &serde_json::Value,
) -> Option<HashMap<String, String>> {
        let mut tracks_href = HashMap::new();
        let items = json_response
                .get("items")
                .and_then(|item| item.as_array())?;

        for item in items.iter() {
                let track_url = item
                        .get("tracks")
                        .and_then(|track| track.get("href"))
                        .and_then(|href| href.as_str())?
                        .to_string();
                let playlist_id = item.get("id").and_then(|id| id.as_str())?.to_string();

                tracks_href.insert(playlist_id, track_url);
        }
        Some(tracks_href)
}

pub(crate) fn build_track(json_track: &serde_json::Value) -> Result<Vec<Track>, SpotiErr> {
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
                        .ok_or(SpotiErr::TrackWithoutId)?
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
                        .ok_or(SpotiErr::TrackWithoutURL)?
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

pub(crate) fn build_playlist(
        mut tracks: HashMap<String, Vec<Track>>,
        json_response: serde_json::Value,
) -> Result<Vec<Playlist>, SpotiErr> {
        let items = json_response
                .get("items")
                .and_then(|item| item.as_array())
                .ok_or(SpotiErr::NoPlaylists)?;

        let mut playlists: Vec<Playlist> = Vec::new();

        for item in items.iter() {
                if let Some(spotify_id) = item.get("id").and_then(|id| id.as_str()) {
                        let name = item
                                .get("name")
                                .and_then(|name| name.as_str())
                                .unwrap_or("unknown")
                                .to_string();

                        if let Some(tracks) = tracks.remove(spotify_id) {
                                playlists.push(Playlist {
                                        spotify_id: spotify_id.to_string(),
                                        name,
                                        tracks,
                                });
                        };
                }
        }

        Ok(playlists)
}
