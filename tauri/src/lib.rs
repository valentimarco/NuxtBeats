mod api;
mod cookie;
mod error;
mod commands;

use std::sync::Arc;

#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri::{
    async_runtime::Mutex,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
     Manager, Wry,
};
use tauri_plugin_store::{Store, StoreExt};

use tauri_specta::{collect_commands, Builder};

use crate::commands::init::{get_ytmusic_cookies, instance_ytmusic_api, logout_ytmusic};

use crate::{api::YoutubeMusicApi, error::Result};

struct AppState {
    store: Mutex<Arc<Store<Wry>>>,
    api: Mutex<Option<YoutubeMusicApi>>,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder_specta = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        // .typ::<command::Playlist>()
        // .events(collect_events![Auth])
        .commands(collect_commands![get_ytmusic_cookies, instance_ytmusic_api, logout_ytmusic]);
    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder_specta
        .export(
            Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
            "../app/utils/tauri.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            //define events
            // builder_specta.mount_events(app);
            let store = Mutex::new(
                app.store(
                    app.path()
                        .app_config_dir()
                        .unwrap()
                        .as_path()
                        .join("store.json"),
                )
                .unwrap(),
            );

            app.manage(AppState { store, api: Mutex::new(None) });

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    other => {
                        println!("menu item {} not handled", other);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(builder_specta.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
