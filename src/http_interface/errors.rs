use crate::shpotify::errors::ShpotifyErrors;
use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpInterfaceErrors {
        #[error("Missing query params")]
        MissingQueryParam,
        #[error(transparent)]
        ShpotifyError(#[from] ShpotifyErrors),
        #[error("Playlist has no tracks")]
        EmptyPlaylist,
}

impl IntoResponse for HttpInterfaceErrors {
        fn into_response(self) -> Response {
                let (status, message) = match self {
                        HttpInterfaceErrors::MissingQueryParam => {
                                (StatusCode::BAD_REQUEST, self.to_string())
                        }
                        HttpInterfaceErrors::ShpotifyError(spotify_apierrors) => {
                                (StatusCode::BAD_REQUEST, spotify_apierrors.to_string())
                        }
                        HttpInterfaceErrors::EmptyPlaylist => {
                                (StatusCode::BAD_REQUEST, self.to_string())
                        }
                };

                (status, message).into_response()
        }
}
