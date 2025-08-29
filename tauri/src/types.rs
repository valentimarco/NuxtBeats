use serde::Serialize;
use specta::Type;

#[derive(Serialize, Type)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub cover: Vec<String>,
    pub tracks: u64,
}
