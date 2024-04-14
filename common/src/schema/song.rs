use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
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

/// A song object (for the database)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Songs {
    pub song_id: uuid::Uuid,
    pub title: String,
    pub artist_id: uuid::Uuid,
    pub album_id: uuid::Uuid,
    pub duration: u16,
    pub genre: Genre,
    pub external_url: Vec<String>,
}

/// A song object (for the client)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    /// The song title
    pub title: String,
    /// The artist name
    pub artist: String,
    /// The album name
    pub album: String,
    /// The song duration in seconds
    pub duration: u16,
    /// The album cover URL
    pub cover: String,
    /// The song URL
    pub url: String,
}