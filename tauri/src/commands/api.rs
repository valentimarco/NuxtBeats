use tauri::State;

use crate::{
    types::{Playlist, Song},
    AppState, Result,
};

#[tauri::command]
#[specta::specta]
pub async fn get_all_playlists(state: State<'_, AppState>) -> Result<Vec<Playlist>> {
    let guard = state.api.lock().await;
    let api = guard.as_ref().unwrap();

    Ok(api.get_all_playlists().await?)
}

#[tauri::command]
#[specta::specta]
pub async fn get_songs_from_playlist(
    state: State<'_, AppState>,
    playlist_id: String,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<Vec<Song>> {
    let guard = state.api.lock().await;
    let api = guard.as_ref().unwrap();
    // Fallback bc google is shit and
    // if you have playlists created even with only ytmusic items, the ytmusic api will crash...
    return match api
        .get_songs_from_playlist_music(playlist_id.clone(), offset, limit)
        .await
    {
        Ok(x) => Ok(x),
        Err(_) => Ok(api
            .get_songs_from_playlist_yt(playlist_id.clone(), offset, limit)
            .await?),
    };
}
