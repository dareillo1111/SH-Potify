use axum::{
        body::Body,
        extract::Query,
        http::{HeaderMap, Response, StatusCode},
        response::IntoResponse,
};
use serde::Deserialize;
use std::{
        fs::File,
        io::{Read, Seek, SeekFrom},
};

pub mod cache_manager;
pub mod token_manager;
pub mod spoty_queries;

#[derive(Deserialize)]
pub struct GetSongParams {
        id: String,
}

const SONG_PATH: &str = "/home/dario/code/rust/TFG/tokio_test/test_audio/rick.mp3";

pub async fn get_song_range(
        Query(params): Query<GetSongParams>,
        headers: HeaderMap,
) -> impl IntoResponse {
        let id = params.id;

        let mut file = File::open(SONG_PATH).unwrap();
        let mut bytes: Vec<u8> = Vec::new();

        if let Some(offset_header) = headers.get("range") {
                if let Ok(offset_str) = offset_header.to_str() {
                        if let Some(offset) = parse_offset_from_str(offset_str) {
                                file.seek(SeekFrom::Start(offset)).unwrap();
                        }
                }
        }

        file.read_to_end(&mut bytes).unwrap();

        Response::builder()
                .status(StatusCode::PARTIAL_CONTENT)
                .body(Body::from(bytes))
                .unwrap()
}

fn parse_offset_from_str(offset: &str) -> Option<u64> {
        let length = offset.len();
        let bytes_str = &offset[6..length - 1];
        if let Ok(bytes) = bytes_str.parse::<u64>() {
                Some(bytes)
        } else {
                None
        }
}
