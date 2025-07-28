use axum::{extract::Query, http::{header, Response}, response::IntoResponse, Json};
use mp3_metadata::read_from_file;
use serde::{Deserialize, Serialize};
use std::{
        collections::HashMap,
        fs::File,
        io::{Read, Seek, SeekFrom},
        path::Path,
        time::Duration,
};

#[derive(Deserialize)]
pub struct MetadataParams {
        song_id: String,
}

#[derive(Deserialize)]
pub struct ChunkParams {
        song_id: String,
        chunk: usize,
}

#[derive(Debug, Serialize)]
pub struct HttpMp3Metadata {
        duration: u128,
        title: String,
        artist: String,
        album: String,
}

const NO_VALUE: &str = "NULL";
const CHUNK_SIZE: usize = 255;
const PATH: &str = "/home/dario/code/rust/TFG/tokio_test/test_audio/rick.mp3";

pub async fn get_song_metadata(Query(params): Query<MetadataParams>) -> Json<HttpMp3Metadata> {
        let path = Path::new(PATH);
        let metadata = read_from_file(path).unwrap();

        println!("{:?}", metadata.tag);

        let http_metadata = HttpMp3Metadata {
                duration: metadata.duration.as_millis(),
                title: metadata
                        .tag
                        .as_ref()
                        .map(|t| t.title.clone())
                        .unwrap_or(NO_VALUE.to_owned()),
                artist: metadata
                        .tag
                        .as_ref()
                        .map(|t| t.artist.clone())
                        .unwrap_or(NO_VALUE.to_owned()),
                album: metadata
                        .tag
                        .as_ref()
                        .map(|t| t.album.clone())
                        .unwrap_or(NO_VALUE.to_owned()),
        };

        println!("Asked for metadata:\n {:?}", http_metadata);

        Json(http_metadata)
}

pub async fn get_song_chunk(Query(params): Query<ChunkParams>) -> impl IntoResponse {
        let path = Path::new(PATH);
        let mut mp3_file = File::open(path).unwrap();
        let song_id = params.song_id;
        let chunk = params.chunk;

        println!("chunk: {song_id} & {chunk}");

        let frames = read_from_file(path).unwrap().frames;
        let chunk_offset = chunk * CHUNK_SIZE;

        let mut out_frames: Vec<u8> = Vec::new();

        for i in chunk_offset..chunk_offset + CHUNK_SIZE {
                if let Some(frame) = frames.get(i) {
                        let offset = frame.offset;
                        let size = frame.size;
                        let mut frame_buf = vec![0u8; size as usize];

                        mp3_file.seek(SeekFrom::Start(offset as u64))
                                .expect("Error en seek");
                        mp3_file.read_exact(&mut frame_buf)
                                .expect("Error leyendo el chunk");

                        out_frames.extend(frame_buf);
                } else {
                        break;
                }
        }

        println!("asked for chunk:{chunk}\n chunk_offset:{chunk_offset}");

        Response::builder()
                .header(header::CONTENT_TYPE, "audio/mpeg")
                .body(axum::body::Body::from(out_frames))
                .unwrap()
}

pub async fn get_chunk_duration(Query(params): Query<ChunkParams>) -> Json<u128> {
        let path = Path::new(PATH);
        let mut mp3_file = File::open(path).unwrap();
        let song_id = params.song_id;
        let chunk = params.chunk;

        println!("duration: {song_id} & {chunk}");

        let frames = read_from_file(path).unwrap().frames;
        let chunk_offset = chunk * CHUNK_SIZE;

        let mut chunk_duration: u128  = 0;

        for i in chunk_offset..chunk_offset + CHUNK_SIZE {
                if let Some(frame) = frames.get(i) {
                        chunk_duration += frame.duration.unwrap().as_millis();
                } else {
                        break;
                }
        }

        println!("chunk: {chunk} --- duration: {chunk_duration}");
        
        Json(chunk_duration)
}
