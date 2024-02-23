use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Users {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub preferred_platform: Option<String>,
    pub photo: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Songs {
    song_id: uuid::Uuid,
    title: String,
    artist_id: uuid::Uuid,
    album_id: uuid::Uuid,
    duration: u16,
    genre: String,
    external_url: Vec<String>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Artists {
    artist_id: uuid::Uuid,
    name: String,
    genre: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Albums {
    album_id: uuid::Uuid,
    title: String,
    artist_id: uuid::Uuid,
    release_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UserPreferences {
    preference_id: uuid::Uuid,
    user_id: uuid::Uuid,
    song_id: uuid::Uuid,
    artist_id: uuid::Uuid,
    album_id: uuid::Uuid,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Recommendations {
    recommendation_id: uuid::Uuid,
    user_id: uuid::Uuid,
    song_id: uuid::Uuid,
    match_score: f32,
}