// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod tray;

use tray::{get_tray, handle_tray_event};

fn main() {
    let tray = get_tray();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(handle_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
