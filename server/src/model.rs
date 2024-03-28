use chrono::prelude::*;
use common::schema::select::SelectItem;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator; 
use strum_macros::EnumIter;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "platform", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Platform {
    AppleMusic,
    Spotify,
    Soundcloud,
    YoutubeMusic,
    AmazonMusic,
    Tidal,
}

pub fn get_platform_select_items() -> Vec<SelectItem> {
    Platform::iter().map(|platform| {
        let platform_str = format!("{:?}", platform); // Convert the enum variant to a string
        SelectItem::new(&platform_str, &platform_str)
    }).collect()
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::AppleMusic => write!(f, "Apple Music"),
            Platform::Spotify => write!(f, "Spotify"),
            Platform::Soundcloud => write!(f, "SoundCloud"),
            Platform::YoutubeMusic => write!(f, "Youtube Music"),
            Platform::AmazonMusic => write!(f, "Amazon Music"),
            Platform::Tidal => write!(f, "Tidal"),
        }
    }
}

impl From<String> for Platform {
    fn from(s: String) -> Self {
        match s.to_uppercase().replace(" ", "_").as_str() {
            "APPLE_MUSIC" => Platform::AppleMusic,
            "SPOTIFY" => Platform::Spotify,
            "SOUNDCLOUD" => Platform::Soundcloud,
            "YOUTUBE_MUSIC" => Platform::YoutubeMusic,
            "AMAZON_MUSIC" => Platform::AmazonMusic,
            "TIDAL" => Platform::Tidal,
            _ => Platform::Spotify,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum Genre {
    Pop,
    Rock,
    HipHop,
    Rap,
    Jazz,
    Classical,
    Country,
    Electronic,
    Dance,
    RnB,
    Soul,
    Reggae,
    Folk,
    Blues,
    Latin,
    Metal,
    Punk,
    Indie,
    Alternative,
    World,
    KPop,
    Anime,
    Children,
    Holiday,
}

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
pub struct Platforms {
    pub platform_id: uuid::Uuid,
    pub name: Platform,
    pub url: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Songs {
    song_id: uuid::Uuid,
    title: String,
    artist_id: uuid::Uuid,
    album_id: uuid::Uuid,
    duration: u16,
    genre: Genre,
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