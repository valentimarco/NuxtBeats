use api::MusicApi;
use specta_typescript::Typescript;
use tauri::{AppHandle, Manager};
use tauri_specta::{collect_commands, Builder};

use tokio::sync::Mutex;
mod api;
mod command;
mod error;
mod handler;
mod player;

#[derive(Clone)]
struct AppState {
    music_api: MusicApi,
}

// type State = tauri::State<'a, Mutex<AppState>>;

async fn setup(app: &AppHandle) {
    let music_api = MusicApi::new().await;
    app.manage(Mutex::new(AppState { music_api }));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .typ::<command::Playlist>()
        .commands(collect_commands![command::get_playlists,]);
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
            "../app/utils/tauri.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        // and finally tell Tauri how to invoke them
        .setup(move |app| {
            // This is also required if you want to use events
            //builder.mount_events(app);

            // Setup app
            tauri::async_runtime::block_on(setup(app.handle()));
            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        // on an actual app, remove the string argument
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // tauri::Builder::default()
    //     .plugin(tauri_plugin_shell::init())
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
