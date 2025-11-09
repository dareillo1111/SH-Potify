use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpotifyAPIErrors {
        #[error("No playlist found")]
        NoPlaylists,
        #[error("Empty playlist")]
        EmptyPlaylist,
        #[error("Request to spotify api failed")]
        RequestFailed(#[from] reqwest::Error),
        #[error("User id not found")]
        UserIdNotFound,
}
