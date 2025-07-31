use std::sync::Mutex;

use once_cell::sync::Lazy;
use sqlx::{Error, Connection, SqliteConnection};

use crate::db_queries::init_script::init;
mod init_script;

const DB_URL: &str = "db/potify.db";
static CONNECTION: Lazy<Mutex<Option<SqliteConnection>>> = Lazy::new(|| Mutex::new(None));

pub async fn initiate_db() -> Result<(), sqlx::Error> {
        let conn = SqliteConnection::connect(DB_URL).await?;
        {
                if let Ok(mut guard) = CONNECTION.lock(){
                        *guard = Some(conn); 
                        init(guard.as_mut().unwrap()).await?;

                } else {
                        return Err(Error::InvalidArgument("Failed to open db".to_string()));
                }
        }
        Ok(())
}
