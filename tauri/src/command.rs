use specta::Type;
use tokio::sync::Mutex;

use serde::Serialize;
use tauri::State;

use crate::{error::Result, AppState};

#[derive(Serialize, Type)]
pub struct Playlist {
    id: String,
    name: String,
    description: String,
    cover: Vec<String>,
    tracks: u64
}

#[tauri::command]
#[specta::specta]
pub async fn get_playlists(state: State<'_, Mutex<AppState>>) -> Result<Vec<Playlist>> {
    let api = &state.lock().await.music_api;
    let playlists = api.get_all_playlists().await;

    let mut output = Vec::new();
    for playlist in playlists {
        output.push(Playlist {
            id: playlist.id,
            name: playlist.name,
            description: String::from(""),
            tracks: playlist.track_count.unwrap_or(0),
            cover: playlist.thumbnail.iter().map(|i| i.url.clone()).collect(),
        });
    }

    Ok(output)
}
