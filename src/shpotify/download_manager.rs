use crate::{shpotify::music_entities::Track, track_downloader};

pub(crate) async fn download_track(track: &mut Track) -> Option<()> {
        if let Some(track_path) = track_downloader::download_track(track).await {
                track.file_path = Some(track_path);
                Some(())
        } else {
                track.file_path = None;
                None
        }
}
