use axum::{Router, routing::get};
use serde_json::Value;
use std::env;
use std::fs;
use std::process::Command;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/api/songs", get(get_song_names));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_song_names() -> Value {
    let songs = fs::read_dir("songs").unwrap();
    songs
}

async fn handler() -> &'static str {
    "Hellolooo"
}

fn download_song_from_youtube() {
    let args: Vec<String> = env::args().collect();
    let video = &args[1];
    println!("Youtube video to download: {video}");
    Command::new("yt-dlp_macos")
        .arg("-f")
        .arg("bestaudio")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("mp3")
        .arg(video)
        .arg("--output")
        .arg("songs/%(title)s.%(ext)s")
        .status()
        .expect("Failed to execute command");
}
