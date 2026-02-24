use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerState {
    pub playing: bool,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub artwork_url: Option<String>,
    pub last_updated: DateTime<Utc>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            playing: false,
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            artwork_url: None,
            last_updated: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingPlayerState {
    pub playing: bool,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub artwork_url: Option<String>,
}

impl From<IncomingPlayerState> for PlayerState {
    fn from(value: IncomingPlayerState) -> Self {
        Self {
            playing: value.playing,
            title: value.title.unwrap_or_default(),
            artist: value.artist.unwrap_or_default(),
            album: value.album.unwrap_or_default(),
            artwork_url: value.artwork_url,
            last_updated: Utc::now(),
        }
    }
}
