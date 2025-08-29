use std::sync::Arc;

use axum::{
        routing::{get, post},
        Router,
};
use http_interface::*;
use sqlx::SqlitePool;
use tokio::sync::Semaphore;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod http_interface;
mod shpotify;
mod spotify_api;
mod track_downloader;
mod track_streamer;

#[derive(Clone)]
pub struct AppState {
        db_pool: SqlitePool,
        semaphore: Arc<Semaphore>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
        let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any);

        let db_pool = db::init_db("url").await.expect("Failed initializing db");

        let semaphore = Arc::new(Semaphore::new(8));

        let state = AppState {
                db_pool,
                semaphore,
        };

        let router = Router::new()
                .route("/select_db_playlists", get(select_db_playlists))
                .route("/get_user_playlists", get(get_user_playlists))
                .route("/download_playlists", post(download_playlists))
                .route("/stream/{song_id}", get(stream_track))
                .layer(cors)
                .with_state(state);

        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}
