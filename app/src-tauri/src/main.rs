#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use anyhow::Result;
use rocket::routes;
use tauri::{
    ActivationPolicy, AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use crate::routes::{debug::{show, hide, quit, status}, install_boost, all_options};

mod routes;
mod cors;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
            "quit" => {
                app.exit(0);
            }
            "hide" => {
                app.hide().unwrap();
            }
            "show" => {
                app.show().unwrap();
            }
            _ => todo!(),
        },
        _ => println!("Unhandled system tray event"),
    }
}

fn main() -> Result<()> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(show);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            // Hides the dock icon on macOS. Barely documented, see
            // https://github.com/tauri-apps/tauri/issues/4852#issuecomment-1312716378
            app.set_activation_policy(ActivationPolicy::Accessory);

            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let ignite = rocket::build()
                    .manage(handle)
                    .mount("/", routes![show, hide, quit, status, install_boost, all_options])
                    .attach(cors::CORS)
                    .ignite()
                    .await
                    .unwrap();

                ignite.launch().await.unwrap();
            });

            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => (),
        });

    Ok(())
}
