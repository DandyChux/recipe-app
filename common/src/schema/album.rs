use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Album {
    album_id: uuid::Uuid,
    title: String,
    artist_id: uuid::Uuid,
    release_date: DateTime<Utc>,
}