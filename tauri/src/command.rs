use specta::Type;
use tokio::sync::Mutex;

use serde::Serialize;
use tauri::State;

use crate::{error::Result, AppState};

#[derive(Serialize, Type)]
pub struct Playlists {
    id: String,
    name: String,
    description: String,
    cover: Vec<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_playlists(state: State<'_, Mutex<AppState>>) -> Result<Vec<Playlists>> {
    let api = &state.lock().await.music_api;
    let playlists = api.get_all_playlists().await;

    let mut output = Vec::new();
    for playlist in playlists {
        output.push(Playlists {
            id: playlist.id,
            name: playlist.name,
            description: String::from(""),
            cover: playlist.thumbnail.iter().map(|i| i.url.clone()).collect(),
        });
    }

    Ok(output)
}
