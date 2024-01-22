use tauri::{AppHandle, LogicalSize, Manager, Size, SystemTrayEvent, WindowBuilder, WindowUrl};
use tauri_plugin_positioner::{self, Position, WindowExt};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

fn setup_vibrancy(window: &tauri::Window) -> Result<(), ()> {
    #[cfg(target_os = "macos")]
    let result = apply_vibrancy(
        &window,
        NSVisualEffectMaterial::HudWindow,
        Some(NSVisualEffectState::Active),
        None,
    );

    if result.is_err() {
        return Err(());
    }

    Ok(())
}

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);

    let (menu, is_built) = {
        let window = app.get_window("menu");

        if let Some(win) = window {
            (win, false)
        } else {
            let win = WindowBuilder::new(app, "menu", WindowUrl::App("system-tray".into()))
                .decorations(false)
                .transparent(true)
                .build()
                .unwrap();

            win.set_size(Size::Logical(LogicalSize {
                width: 300.0,
                height: 600.0,
            }))
            .expect("Could not set window size");

            setup_vibrancy(&win).expect("Could not setup vibrancy");

            (win, true)
        }
    };

    if menu.is_visible().unwrap() && !is_built {
        menu.hide().expect("failed to hide window");
    } else {
        menu.move_window(Position::TrayLeft)
            .expect("Could not move window");
        menu.show().expect("Could not show window");
        menu.set_focus().expect("Could not focus window");
        menu.set_always_on_top(true)
            .expect("Could not setup always on top");
    }
}
