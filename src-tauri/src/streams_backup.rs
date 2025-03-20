use axum::{
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router, extract::Path,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use lofty::{read_from_path, TaggedFileExt};
use rusqlite::{Connection, params};
use tower_http::services::ServeDir;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Song {
    id: i32,
    title: String,
    artist: String,
    album: String,
    file_path: String,
}

async fn list_songs(conn: &Connection) -> Result<Json<Vec<Song>>, StatusCode> {
    let mut stmt = conn.prepare("SELECT id, title, artist, album, file_path FROM songs").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let song_iter = stmt.query_map(params![], |row| {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            album: row.get(3)?,
            file_path: row.get(4)?,
        })
    }).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut songs = Vec::new();
    for song in song_iter {
        songs.push(song.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?);
    }
    Ok(Json(songs))
}

async fn get_song(Path(id): Path<i32>, conn: &Connection) -> Result<Json<Song>, StatusCode> {
    let mut stmt = conn.prepare("SELECT id, title, artist, album, file_path FROM songs WHERE id = ?").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let song = stmt.query_row(params![id], |row| {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            album: row.get(3)?,
            file_path: row.get(4)?,
        })
    }).map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(song))
}

async fn stream_song(Path(id): Path<i32>, conn: &Connection) -> Result<Response, StatusCode> {
    let mut stmt = conn.prepare("SELECT file_path FROM songs WHERE id = ?").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_path: String = stmt.query_row(params![id], |row| row.get(0)).map_err(|_| StatusCode::NOT_FOUND)?;

    let mut file = File::open(file_path).map_err(|_| StatusCode::NOT_FOUND)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = axum::body::Full::from(buffer);
    let content_type = mime_guess::from_path(file_path).first_or_octet_stream();
    Ok(Response::builder()
        .header("Content-Type", content_type.as_ref())
        .body(body)
        .unwrap())
}

async fn scan_directory(conn: &Connection) -> Result<String, StatusCode> {
    let dir_path = "/path/to/your/music/directory"; // Replace with your music directory
    let paths = std::fs::read_dir(dir_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for path in paths {
        if let Ok(entry) = path {
            let file_path = entry.path();
            if let Some(ext) = file_path.extension() {
                if ext == "mp3" || ext == "flac" {
                    if let Ok(tagged_file) = read_from_path(&file_path) {
                        if let Some(tag) = tagged_file.primary_tag() {
                            let title = tag.title().unwrap_or("Unknown Title").to_string();
                            let artist = tag.artist().unwrap_or("Unknown Artist").to_string();
                            let album = tag.album().unwrap_or("Unknown Album").to_string();
                            let file_path_str = file_path.to_str().unwrap_or("").to_string();

                            conn.execute(
                                "INSERT INTO songs (title, artist, album, file_path) VALUES (?1, ?2, ?3, ?4)",
                                params![title, artist, album, file_path_str],
                            ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                        }
                    }
                }
            }
        }
    }
    Ok("Scan completed".to_string())
}

#[tokio::main]
async fn main() {
    let conn = Connection::open("music.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS songs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            album TEXT NOT NULL,
            file_path TEXT NOT NULL UNIQUE
        )",
        [],
    ).unwrap();

    let app = Router::new()
        .route("/songs", get(|| async {
            list_songs(&conn).await
        }))
        .route("/songs/:id", get(|path| async {
            get_song(path, &conn).await
        }))
        .route("/stream/:id", get(|path| async {
            stream_song(path, &conn).await
        }))
        .route("/scan", post(|| async {
            scan_directory(&conn).await
        }))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
