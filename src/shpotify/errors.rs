use crate::{db::errors::DbErrors, spotify_api::errors::SpotifyAPIErrors};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShpotifyErrors {
        #[error(transparent)]
        SpotifyAPIError(#[from] SpotifyAPIErrors),
        #[error(transparent)]
        DbError(#[from] DbErrors),
}
