#![allow(unused)]
use axum::{
        body::Body, extract::Query, http::{header, Response}, response::IntoResponse, routing::{get, get_service}, Router
};
use mp3_metadata::{read_from_file, Frame};
use serde::Deserialize;
use std::{
        collections::HashMap, fmt::format, fs::File, io::{BufRead, BufReader, Read, Seek, SeekFrom}, path::Path
};
use tokio::fs;
use tower_http::services::{self, ServeFile};

mod mp3_querys;


#[tokio::main]
async fn main() {
        start_server().await;
}

async fn start_server() {
        let router = Router::new()
                .nest_service("/song_chunk_duration", get(mp3_querys::get_chunk_duration))
                .nest_service("/song_chunk", get(mp3_querys::get_song_chunk))
                .nest_service("/song_metadata", get(mp3_querys::get_song_metadata));

        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}
