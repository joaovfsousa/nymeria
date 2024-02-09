const LIGHT_DEVICE_ADDRESS: &'static str = "http://192.168.0.150:80";

fn get_url(path: &String) -> String {
    let mut url = LIGHT_DEVICE_ADDRESS.to_owned();

    url.push_str("/");
    url.push_str(path);

    url
}

fn get_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::new()
}

pub fn set_device_state(device_id: String, state: String) {
    println!("Setting device {} to state {}", device_id, state.as_str());
    let client = get_client();

    let mut path = "devices/".to_string();

    path.push_str(&device_id);

    path.push_str("/state");

    let url = get_url(&path);

    client
        .post(url)
        .body(state.clone())
        .send()
        .expect("Failed to set device state");
}

pub fn reset() {
    println!("Resetting device");

    let client = get_client();

    let url = get_url(&"reset".to_string());

    client.post(url).send().expect("Failed to reset");
}

pub fn get_state() -> String {
    println!("Reading state");

    let client = get_client();

    let url = get_url(&"state".to_string());

    let res = client.get(url).send().expect("Failed to read state");

    let state = res.text();

    println!("State: {}", state.as_ref().unwrap());

    state.unwrap()
}
