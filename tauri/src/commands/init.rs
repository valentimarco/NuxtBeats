use serde_json::json;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;

use crate::api::YoutubeMusicApi;
use crate::{Result, error::Error};
use crate::AppState;
use crate::cookie::cookies_to_netscape_format;

#[tauri::command]
#[specta::specta]
pub async fn get_ytmusic_cookies(app_handle: AppHandle, label: String) -> Result<()> {
    let state = app_handle.state::<AppState>();
    let store = state.store.lock().await;
    let webview = app_handle
        .get_webview_window(&label)
        .ok_or(Error::Tauri(String::from("Error when getting webview")))?;
    let res;
    #[cfg(target_os = "windows")]
    {
        res = tauri::async_runtime::spawn(async move {
            webview.cookies().map(|x| cookies_to_netscape_format(x))
        })
        .await
        .map_err(|err| Error::from(err))??;
    }
    #[cfg(not(target_os = "windows"))]
    {
        res = webview.cookies().map(|x| cookies_to_netscape_format(x))?
    }

    store.set("cookies", json!(res));
    store.save().map_err(|err| Error::Tauri(err.to_string()))?;
    instance_ytmusic_api(app_handle.clone(), res).await.map_err(|err| Error::IO(err.to_string()))
}

#[tauri::command]
#[specta::specta]
pub async fn instance_ytmusic_api(app_handle: AppHandle, cookies: String) -> Result<()> {
    let state = app_handle.state::<AppState>();
    let config_dir = app_handle.path().app_config_dir().unwrap();
    match YoutubeMusicApi::new(cookies, &config_dir).await {
        Ok(x) => {
            let mut api = state.api.lock().await;
            if api.is_none() {
                *api = Some(x);
            }
            app_handle.emit("auth:login", ()).unwrap();
            Ok(())
        }
        Err(err) => {
            Err(err)
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn logout_ytmusic(app_handle: AppHandle) -> Result<()> {
    let state = app_handle.state::<AppState>();
    let mut api = state.api.lock().await;
    if let Some(x) = api.as_ref() {
        let _ = x.logout().await;
        *api = None;
        app_handle.emit("auth:logout", ()).unwrap();
    }
    Ok(())
}