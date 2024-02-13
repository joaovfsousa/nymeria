use std::fmt;

use crate::state_manager::State;

const LIGHT_DEVICE_ADDRESS: &'static str = "http://192.168.0.150:80";

pub struct SignClient {
    client: reqwest::blocking::Client,
}

pub type Callback = Option<fn()>;

pub enum Device {
    MacMic,
    MacTray,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Device::MacMic => write!(f, "macmic"),
            Device::MacTray => write!(f, "mactray"),
        }
    }
}

impl SignClient {
    pub fn new() -> SignClient {
        SignClient {
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_url(path: &String) -> String {
        let mut url = LIGHT_DEVICE_ADDRESS.to_owned();

        url.push_str("/");
        url.push_str(path);

        url
    }

    pub fn set_device_state(&self, device: Device, state: State, callback: Callback) {
        let device_id = device.to_string();
        let state_as_string = state.to_string();

        println!("Setting device {} to state {}", device_id, state_as_string);

        let mut path = "devices/".to_string();

        path.push_str(&device_id);

        path.push_str("/state");

        let url = SignClient::get_url(&path);

        self.client
            .post(url)
            .body(state_as_string)
            .send()
            .expect("Failed to set device state");

        if let Some(cb) = callback {
            cb();
        }
    }

    pub fn reset(&self, callback: Callback) {
        println!("Resetting device");

        let url = SignClient::get_url(&"reset".to_string());

        self.client.post(url).send().expect("Failed to reset");

        if let Some(cb) = callback {
            cb();
        }
    }

    pub fn get_state(&self, callback: Callback) -> State {
        println!("Reading state");

        let url = SignClient::get_url(&"state".to_string());

        let res = self.client.get(url).send().expect("Failed to read state");

        let state = res.text();

        println!("State: {}", state.as_ref().unwrap());

        if let Some(cb) = callback {
            cb();
        };

        State::from(state.unwrap())
    }
}
