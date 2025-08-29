use crate::{db::errors::DbErrors, spotify_api::errors::SpotifyAPIErrors};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShpotifyErrors {
        #[error(transparent)]
        SpotifyAPIError(#[from] SpotifyAPIErrors),
        #[error("Failed downloading track")]
        DownloadingTrackFailed,
        #[error("Failed persisting track")]
        PersistingTrackFailed,
        #[error(transparent)]
        DbError(#[from] DbErrors),
}
