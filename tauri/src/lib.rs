mod cookie;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
    Builder,
    AppHandle
};

use crate::cookie::cookies_to_netscape_format;

#[tauri::command]
async fn get_ytmusic_cookies(app_handle: AppHandle, label: String) -> Result<String, String> {
    let webview = app_handle.get_webview(&label);
    if let Some(webview) = webview {
        let _ = tauri::async_runtime::spawn(async move {
            match webview.cookies() {
                Ok(cookies) => println!("Cookies: {}", cookies_to_netscape_format(cookies)),
                Err(e) => println!("Error getting cookies: {}", e),
            }
        }).await;
        Ok("Cookies obtained successfully".into())
    } else {
        Err(format!("error: No webview found with label: {}", label))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
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
            .setup(|app| {
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
            });
    }

    builder
        .invoke_handler(tauri::generate_handler![get_ytmusic_cookies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
