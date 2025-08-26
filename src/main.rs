use std::env::{self, set_var};

use axum::{
        routing::{get, post},
        Router,
};
use http_interface::*;
use sqlx::SqlitePool;
mod db;
mod http_interface;
mod shpotify;
mod spotify_api;
mod track_downloader;

#[derive(Clone)]
pub struct AppState {
        db_pool: SqlitePool,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
        let connection = db::init_db("url").await.expect("Failed initializing db");
        let state = AppState {
                db_pool: connection,
        };

        let router = Router::new()
                .route("/get_user_playlists", get(get_user_playlists))
                .route("/download_playlists", post(download_playlists))
                .with_state(state);

        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}
