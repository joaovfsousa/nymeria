use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use crate::{
    sign_client::{Device, SignClient},
    state_manager::State,
};

pub fn get_tray() -> SystemTray {
    let free = CustomMenuItem::new("free".to_string(), "🟢 Free".to_string());
    let maybe = CustomMenuItem::new("maybe".to_string(), "🟠 Maybe".to_string());
    let busy = CustomMenuItem::new("busy".to_string(), "🔴 Busy".to_string());
    let reset = CustomMenuItem::new("reset".to_string(), "Reset".to_string());
    let quit = CustomMenuItem::new("quit".to_string(), "Quit".to_string());

    let tray_menu = SystemTrayMenu::new()
        .add_item(free)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(maybe)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(busy)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(reset)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn system_tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let sign_client = SignClient::new();

            match id.as_str() {
                "quit" => {
                    app.exit(0);
                }
                "reset" => {
                    sign_client.get_state();
                    sign_client.reset();
                }
                _ => sign_client.set_device_state(Device::MacTray, State::from(id)),
            }
        }
        _ => {}
    }
}
