// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mic;
mod sign_client;
mod state_manager;
mod tray;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use mic::get_is_mic_in_use;
use sign_client::SignClient;
use state_manager::StateManager;
use tray::{get_system_tray_event_handler, get_tray};

fn main() {
    let tray = get_tray();

    let client = SignClient::new();

    let state_manager = Arc::new(Mutex::new(StateManager::new()));

    let state_check_thread_state_manager = state_manager.clone();

    let mic_check_thread_state_manager = state_manager.clone();

    let _state_check_thread = thread::spawn(move || loop {
        state_check_thread_state_manager
            .lock()
            .unwrap()
            .update_from_sign_state();

        thread::sleep(Duration::from_secs(3));
    });

    let _mic_check_thread = thread::spawn(move || loop {
        let is_mic_in_use = get_is_mic_in_use();

        mic_check_thread_state_manager
            .lock()
            .unwrap()
            .mic_usage_update(is_mic_in_use);

        thread::sleep(Duration::from_secs(3));
    });

    let on_system_tray_event = get_system_tray_event_handler(client);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(on_system_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
