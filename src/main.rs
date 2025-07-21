#![allow(unused)]
use axum::{
        routing::{get, get_service},
        Router,
};
use std::{fs::File, io::{BufRead, BufReader, Read}};
use tokio::fs;
use tower_http::services::{self, ServeFile};

const FILE_PATH: &str = "/home/dario/code/rust/TFG/tokio_test/test_audio/sample-15s.mp3";

#[tokio::main]
async fn main() {
        stream_read_file().await;
}

async fn start_server() {
        let router =
                Router::new().nest_service("/test_audio", get_service(ServeFile::new(FILE_PATH)));
        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}

async fn stream_read_file() {
        let mut file = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);

        for line in reader.bytes(){
                println!("{:?}", line);
        }
}
