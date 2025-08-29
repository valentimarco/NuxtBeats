use tauri::State;

use crate::{types::Playlist, AppState, Result};

#[tauri::command]
#[specta::specta]
pub async fn get_all_playlists(state: State<'_, AppState>) -> Result<Vec<Playlist>> {
    let guard = state.api.lock().await;
    let api = guard.as_ref().unwrap();

    Ok(api.get_all_playlists().await?)
}
