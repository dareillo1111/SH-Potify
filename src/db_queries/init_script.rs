use sqlx::{query, SqliteConnection};


pub async fn init(connection: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        println!("{:?}", connection);

        query("CREATE TABLE IF NOT EXISTS Songs (
                ID int NOT NULL,
                SpotifyId varchar(255) NOT NULL,
                AlbumName varchar(255),
                Name varchar(255) NOT NULL,
                FilePath varchar(255),
                URL varchar(255) NOT NULL,
                PRIMARY KEY (ID)
        )")
        .execute(&mut *connection)
        .await?;

        query("CREATE TABLE IF NOT EXISTS PlayLists (
                ID int NOT NULL,
                Name varchar(255),
                PRIMARY KEY (ID)
        )")
        .execute(&mut *connection)
        .await?;

        Ok(())
}
