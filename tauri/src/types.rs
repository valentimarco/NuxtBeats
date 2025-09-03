use serde::Serialize;
use specta::Type;
use time::Duration;

#[derive(Serialize, Type, Default)]
pub struct Artist {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Type, Default)]
pub struct Album {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Type)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub time: Duration,
}

#[derive(Serialize, Type)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub cover: Vec<String>,
    pub tracks: u64,
}
