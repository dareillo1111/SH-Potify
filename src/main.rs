use crate::mp3_server::get_song_range;
use crate::token_manager::set_token;
use axum::{routing::get, Router};
use tokio;

mod mp3_server;
mod token_manager;

#[tokio::main]
async fn main() {
        set_token().await.unwrap();
        start_server().await;
}

async fn start_server() {
        let router = Router::new()
                .route("/get_song", get(get_song_range));

        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}

