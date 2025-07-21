#![allow(unused)]
use axum::{
        routing::{get, get_service},
        Router,
};
use std::{fmt::format, fs::File, io::{BufRead, BufReader, Read}};
use tokio::fs;
use tower_http::services::{self, ServeFile};

const FILE_PATH: &str = "/home/dario/code/rust/TFG/tokio_test/test_audio/sample-15s.mp3";

#[tokio::main]
async fn main() {
        stream_read_file();
}

async fn start_server() {
        let router =
                Router::new().nest_service("/test_audio", get_service(ServeFile::new(FILE_PATH)));
        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}

fn stream_read_file() {
        let file = File::open(FILE_PATH).unwrap();
        let mut reader = BufReader::new(file);
        let mut file_bytes = Vec::new();
        reader.read_to_end(&mut file_bytes);

        let mut byte_index = 0;
        let mut mp3_header: [u8; 4] = [0; 4];
        for byte in file_bytes {
                let rotation = byte_index % 4;

                if rotation == 3{
                        print_header(mp3_header);
                        mp3_header = [0; 4];
                }

                mp3_header[rotation] = byte;
                byte_index += 1;
        }
}

fn print_header(header: [u8; 4]) {
        let mut out_string = String::new();
        for byte in header.iter(){
                let format = format!("{:08b}", byte);
                out_string.push_str(&format);
        }
        println!("{out_string}")
}
