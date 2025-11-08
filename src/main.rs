use axum::{
    Router,
    extract::{Json, Path},
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
use tokio::{fs::File, io::AsyncReadExt, task};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct DownloadRequest {
    url: String,
}

#[derive(Serialize)]
struct DownloadResponse {
    success: bool,
    message: String,
}

async fn serve_html() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_html))
        .route("/api/songs", get(get_song_names))
        .route("/api/download", post(download_song_from_youtube))
        .route("/songs/{filename}", get(download_song))
        .layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn download_song(
    Path(filename): Path<String>,
) -> Result<Response, (StatusCode, &'static str)> {
    // Prevent directory traversal attacks
    if filename.contains("..") || filename.contains('/') {
        return Err((StatusCode::BAD_REQUEST, "Invalid filename"));
    }

    let path = PathBuf::from("songs").join(&filename);

    let mut file = File::open(&path)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "File not found"))?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file"))?;

    let content_disposition = format!("attachment; filename=\"{}\"", filename);

    Ok((
        [
            (header::CONTENT_TYPE, "audio/mpeg"),
            (header::CONTENT_DISPOSITION, content_disposition.as_str()),
        ],
        contents,
    )
        .into_response())
}

async fn get_song_names() -> Result<Json<serde_json::Value>, StatusCode> {
    let mut entries = tokio::fs::read_dir("songs")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut song_names = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        if let Some(name) = entry.file_name().to_str() {
            song_names.push(name.to_string());
        }
    }

    Ok(Json(serde_json::json!({"songs": song_names})))
}


async fn download_song_from_youtube(
    Json(payload): Json<DownloadRequest>,
) -> (StatusCode, Json<DownloadResponse>) {
    let video_url = payload.url;
    println!("Youtube video URL is: {}", video_url);
    
    task::spawn(async move {
        let result = Command::new("yt-dlp_macos")
            .args([
                "-f", "bestaudio",
                "--extract-audio",
                "--audio-format", "mp3",
                "--output", "songs/%(title)s.%(ext)s",
                &video_url,
            ])
            .status();
        
        match result {
            Ok(status) if status.success() => {
                println!("Successfully downloaded: {}", video_url);
            }
            Ok(_) => println!("Download failed for {}", video_url),
            Err(e) => println!("Error downloading {}: {}", video_url, e),
        }
    });
    
    (
        StatusCode::ACCEPTED,
        Json(DownloadResponse {
            success: true,
            message: "Download started in the background".to_string(),
        }),
    )
}

