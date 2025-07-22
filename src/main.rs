#![allow(unused)]
use axum::{
        body::Body, http::{header, Response}, response::IntoResponse, routing::{get, get_service}, Router
};
use mp3_metadata::{read_from_file, Frame};
use std::{
        fmt::format,
        fs::File,
        io::{BufRead, BufReader, Read, Seek, SeekFrom},
        path::Path,
};
use tokio::fs;
use tower_http::services::{self, ServeFile};

#[tokio::main]
async fn main() {
        start_server().await;
}

async fn start_server() {

        let router = Router::new().nest_service("/test_audio", get(get_frame_byte_handler));
        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}

async fn get_frame_byte_handler() -> impl IntoResponse {
        let path = Path::new("/home/dario/code/rust/TFG/tokio_test/test_audio/sample-15s.mp3");
        let bytes = get_frame_byte(path);

        ([(header::CONTENT_TYPE, "audio/mpeg")], bytes)
}

fn get_frame_byte(path: &Path) -> Vec<u8> {
        let mut mp3_file = File::open(path).unwrap();
        let frames: Vec<Frame> = get_frames(path);
        let mut frames_bytes: Vec<u8> = Vec::new();

        for i in 0..frames.len() / 9{
                let frame = frames.get(i).unwrap();
                let offset = frame.offset;
                let size = frame.size;
                let mut frame_buf = vec![0u8; size as usize];

                mp3_file.seek(SeekFrom::Start(offset as u64)).unwrap();
                mp3_file.read_exact(&mut frame_buf).unwrap();

                frames_bytes.append(&mut frame_buf);
        }

        frames_bytes
}

fn get_frames(path: &Path) -> Vec<Frame> {
        let metadata = read_from_file(path).unwrap();
        metadata.frames
}
