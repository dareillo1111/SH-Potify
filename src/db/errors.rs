use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbErrors{
        #[error(transparent)]
        SqliteError(#[from] sqlx::Error),
}
