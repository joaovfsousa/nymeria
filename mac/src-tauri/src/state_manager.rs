use std::fmt;

use crate::sign_client::{Device, SignClient};

#[derive(PartialEq)]
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
        let free_string = String::from("free");
        let busy_string = String::from("busy");

        match value {
            free_string => State::Free,
            busy_string => State::Busy,
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
                    return self.update_from_sign_state();
                }
            }
            false => {
                if self.last_local_state == State::Busy {
                    self.sign_client
                        .set_device_state(Device::MacMic, State::Free, None);
                    self.last_local_state = State::Free;
                    return self.update_from_sign_state();
                }
            }
        }

        self.get_state()
    }

    pub fn update_from_sign_state(&mut self) -> &State {
        let state = self.sign_client.get_state(None);

        self.state = state;

        self.get_state()
    }
}
