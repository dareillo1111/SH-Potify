use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbErrors{
        #[error(transparent)]
        SqliteError(sqlx::Error),
        #[error("{0}")]
        TrackStructError(String),
}
