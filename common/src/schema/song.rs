use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    song_id: uuid::Uuid,
    title: String,
    artist_id: uuid::Uuid,
    album_id: uuid::Uuid,
    duration: u16,
    genre: String,
    external_url: Vec<String>,
}