use tauri::{command, AppHandle};

const LIGHT_DEVICE_ADDRESS: &'static str = "http://192.168.0.150:80";

fn get_url(path: &String) -> String {
    let mut url = LIGHT_DEVICE_ADDRESS.to_owned();

    url.push_str("/");
    url.push_str(path);

    url
}

#[command]
pub fn set_device_state(device_id: String, state: String) {
    println!("Setting device {} to state {}", device_id, state.as_str());
    let client = reqwest::blocking::Client::new();

    let path = "change-status".to_string();

    // path.push_str(&device_id);

    let url = get_url(&path);

    client
        .post(url)
        .body(state.clone())
        .send()
        .expect("Failed to send request");
}

#[command]
pub fn get_state() -> String {
    "free".into()
}

#[command]
pub fn quit(app_handle: AppHandle) {
    println!("Quitting");
    app_handle.exit(0)
}
