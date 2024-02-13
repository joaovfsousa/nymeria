const LIGHT_DEVICE_ADDRESS: &'static str = "http://192.168.0.150:80";

pub struct SignClient {
    client: reqwest::blocking::Client,
}

pub type Callback = Option<fn()>;

pub enum Device {
    MacMic,
    MacTray,
}

fn device_to_string(device: Device) -> String {
    match device {
        Device::MacMic => "macmic".to_string(),
        Device::MacTray => "mactray".to_string(),
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

    pub fn set_device_state(&self, device: Device, state: String, callback: Callback) {
        let device_id = device_to_string(device);

        println!("Setting device {} to state {}", device_id, state.as_str());

        let mut path = "devices/".to_string();

        path.push_str(&device_id);

        path.push_str("/state");

        let url = SignClient::get_url(&path);

        self.client
            .post(url)
            .body(state.clone())
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

    pub fn get_state(&self, callback: Callback) -> String {
        println!("Reading state");

        let url = SignClient::get_url(&"state".to_string());

        let res = self.client.get(url).send().expect("Failed to read state");

        let state = res.text();

        println!("State: {}", state.as_ref().unwrap());

        if let Some(cb) = callback {
            cb();
        };

        state.unwrap()
    }
}
