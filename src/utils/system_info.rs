/// System information collection
use std::collections::HashMap;
use reqwest::blocking::Client;

pub fn get_system_info() -> HashMap<String, String> {
    let mut info = HashMap::new();

    info.insert("os".to_string(), std::env::consts::OS.to_string());
    info.insert("arch".to_string(), std::env::consts::ARCH.to_string());
    info.insert("hostname".to_string(), hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string());
    info.insert("public_ip".to_string(), get_public_ip());

    info
}

fn get_public_ip() -> String {
    let client = Client::new();
    match client.get("https://api.ipify.org").send() {
        Ok(response) => response.text().unwrap_or_else(|_| "unknown".to_string()),
        Err(_) => "unknown".to_string(),
    }
}
