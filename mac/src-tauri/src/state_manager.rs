use std::fmt;

use crate::sign_client::{Device, SignClient};

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Free,
    Busy,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Free => write!(f, "free"),
            State::Busy => write!(f, "busy"),
        }
    }
}

impl From<String> for State {
    fn from(value: String) -> Self {
        match value.as_str() {
            "free" => State::Free,
            "busy" => State::Busy,
            _ => State::Free,
        }
    }
}

pub struct StateManager {
    state: State,
    sign_client: SignClient,
    last_local_state: State,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            state: State::Free,
            sign_client: SignClient::new(),
            last_local_state: State::Free,
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn mic_usage_update(&mut self, is_mic_in_use: bool) -> &State {
        match is_mic_in_use {
            true => {
                if self.last_local_state == State::Free {
                    self.sign_client
                        .set_device_state(Device::MacMic, State::Busy, None);
                    self.last_local_state = State::Busy;
                    println!("Mic set to busy");
                    return self.update_from_sign_state();
                }
            }
            false => {
                if self.last_local_state == State::Busy {
                    self.sign_client
                        .set_device_state(Device::MacMic, State::Free, None);
                    self.last_local_state = State::Free;
                    println!("Mic set to free");
                    return self.update_from_sign_state();
                }
            }
        }

        self.get_state()
    }

    pub fn update_from_sign_state(&mut self) -> &State {
        if let Some(state) = self.sign_client.get_state(None) {
            self.state = state;
        }

        self.get_state()
    }
}
