use tauri::{command, AppHandle};

pub enum State {
    Free,
    Maybe,
    Busy,
}

impl State {
    fn from_str(s: &str) -> State {
        match s {
            "free" => State::Free,
            "maybe" => State::Maybe,
            "busy" => State::Busy,
            _ => panic!("Invalid state"),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            State::Free => "free",
            State::Maybe => "maybe",
            State::Busy => "busy",
        }
    }
}

#[command]
pub fn set_device_state(device_id: String, state: String) {
    let state = State::from_str(&state);

    // TODO: implement the server call
    println!("Setting device {} to state {}", device_id, state.as_str());
    ()
}

#[command]
pub fn get_state() -> String {
    // TODO: get the state from the server
    "free".into()
}

#[command]
pub fn quit(app_handle: AppHandle) {
    println!("Quitting");
    app_handle.exit(0)
}
