use crate::sign_client::{Device, SignClient};

pub enum State {
    Free,
    Busy,
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

    pub fn mic_usage_update(&mut self, is_mic_in_use: bool) {
        match is_mic_in_use {
            true => {
                if (self.last_local_state == State::Free) {
                    self.sign_client
                        .set_device_state(Device::MacMic, "busy".to_string(), None);
                }
            }
            false => {
                self.state = State::Free;
            }
        }
    }
}
