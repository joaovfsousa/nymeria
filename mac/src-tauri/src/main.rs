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
use state_manager::{State, StateManager};
use tauri::Icon;
use tray::{get_tray, system_tray_event_handler};

fn main() {
    let tray = get_tray();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let state_manager = Arc::new(Mutex::new(StateManager::new()));

            let state_check_thread_state_manager = state_manager.clone();
            let mic_check_thread_state_manager = state_manager.clone();
            let icon_update_thread_state_manager = state_manager.clone();

            thread::spawn(move || loop {
                state_check_thread_state_manager
                    .lock()
                    .unwrap()
                    .update_from_sign_state();

                thread::sleep(Duration::from_millis(1500));
            });

            thread::spawn(move || loop {
                let is_mic_in_use = get_is_mic_in_use();

                mic_check_thread_state_manager
                    .lock()
                    .unwrap()
                    .mic_usage_update(is_mic_in_use);

                thread::sleep(Duration::from_secs(3));
            });

            let state_to_icon = |state: &State| -> Icon {
                match state {
                    State::Free => Icon::Raw(include_bytes!("../icons/states/free.png").to_vec()),
                    State::Busy => Icon::Raw(include_bytes!("../icons/states/busy.png").to_vec()),
                }
            };

            let handle = app.handle().clone();

            thread::spawn(move || {
                let mut last_state: Option<String> = None;

                let change_icon = move |state: &State| {
                    let icon = state_to_icon(state);
                    handle.tray_handle().set_icon(icon).unwrap();
                };

                loop {
                    let manager = icon_update_thread_state_manager.lock().unwrap();
                    let state = manager.get_state();

                    let x = last_state.clone();

                    if x.is_none() {
                        change_icon(state);
                        last_state = Some(state.to_string());
                    } else {
                        if x.unwrap() != state.to_string() {
                            change_icon(state);
                            last_state = Some(state.to_string());
                        }
                    }
                    thread::sleep(Duration::from_millis(1000));
                }
            });

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(system_tray_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
