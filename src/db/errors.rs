use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum DbErrors{
        #[error("")]
        SqliteError(sqlx::Error),
        #[error("")]
        TrackStructError(String),
}
