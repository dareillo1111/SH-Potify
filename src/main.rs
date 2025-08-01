use axum::{routing::get, Router};
use tokio;
use crate::music_server::get_song_range;
use crate::music_server::spoty_queries::get_user_playlists;
use crate::music_server::track_downloader::download_playlists;

pub mod db_queries;
pub mod music_server;

#[tokio::main]
async fn main() {
        //println!("{:?}", db_queries::initiate_db().await);
        //println!("{:#?}", get_user_playlists("31q7modz4watrvvo4ixehhzibo6y").await.unwrap());
        start_server().await;
}

async fn start_server() {
        let router = Router::new()
                .route("/get_song", get(get_song_range))
                .route("/get_user_playlists", get(get_user_playlists))
                .route("/download_playlists", get(download_playlists));


        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}
