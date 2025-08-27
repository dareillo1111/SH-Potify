use crate::{db::errors::DbErrors, spotify_api::errors::SpotifyAPIErrors};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShpotifyErrors {
        #[error("")]
        SpotifyAPIError(#[from] SpotifyAPIErrors),
        #[error("")]
        DownloadingTrackFailed,
        #[error("")]
        PersistingTrackFailed,
        #[error(transparent)]
        DbError(#[from] DbErrors),
}
