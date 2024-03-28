use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator; 
use strum_macros::EnumIter;
use chrono::prelude::*;
use crate::schema::select::SelectItem;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
        // Convert to "Title Case" for the label
        let label = platform_str.chars().enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && c.is_uppercase() {
                    vec![' ', c]
                } else {
                    vec![c]
                }
            })
            .collect::<String>();

        // Convert to "SCREAMING_SNAKE_CASE" for the value
        let value = platform_str.chars().enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && c.is_uppercase() {
                    vec!['_', c]
                } else {
                    vec![c]
                }
            })
            .collect::<String>()
            .to_uppercase();

        SelectItem::new(&label, &value)
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