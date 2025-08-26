use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpotifyAPIErrors {
        #[error("No playlist found")]
        NoPlaylists,
        #[error("No url found for this track")]
        TrackWithoutURL,
        #[error("No id found for this track")]
        TrackWithoutId,
        #[error("Empty playlist")]
        EmptyPlaylist,
        #[error("Caching token failed")]
        CachingTokenFailed,
        #[error("Request to spotify api failed")]
        RequestFailed(reqwest::Error),
        #[error("Failed parsing response into json")]
        JsonParsingFailed(reqwest::Error),
        #[error("User id not found")]
        UserIdNotFound,
}
