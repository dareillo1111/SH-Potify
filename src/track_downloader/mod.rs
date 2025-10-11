use std::path::Path;

use crate::shpotify::music_entities::Track;
use tokio::process::Command;

mod errors;

const OUTPUT_DIR: &str = "./tracks/";

pub(crate) async fn download_track(track: &Track) -> Option<String> {
        let spotdl_path = format!("{OUTPUT_DIR}{}.{{output-ext}}", &track.spotify_id);
        let path = format!("{OUTPUT_DIR}{}.mp3", &track.spotify_id);

        // Spotdl CAN'T download songs with age restriction from youtube
        let command = Command::new("/home/dario/.local/bin/spotdl")
                .arg(&track.url)
                .arg("--output")
                .arg(spotdl_path)
                .arg("--format")
                .arg("mp3")
                .arg("--lyrics")
                .arg("genius")
                .arg("--dont-filter-results")
                .status()
                .await
                .expect("downloading track failed");

        // Spotdl always return exitstatus 0 even if there was an error
        // we have to check if the file exists to know if it worked.
        if Path::new(&path).exists() {
                return Some(path);
        }

        None
}
