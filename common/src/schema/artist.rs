use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Artist {
    artist_id: uuid::Uuid,
    name: String,
    genre: String,
}