use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: Ipv4Addr,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Deserialize, Serialize)]
pub struct DeviceInfo {
    pub hostname: Option<String>,
    pub mac_address: Option<Mac>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Mac(pub String);

const MAC_CHARACTERS: &str = "ABCDEF0123456789:";

impl Mac {
    pub fn new(mac: &str) -> Option<Self> {
        if !mac.is_ascii() || !mac.chars().all(|c| MAC_CHARACTERS.contains(c)) {
            return None;
        }

        let len = mac.len();

        if mac.contains(':') && len == 17 {
            Some(Mac(mac.to_string()))
        } else if len == 12 {
            let mut formatted = String::new();

            for i in 0..=5 {
                let start = 2 * i;
                let end = start + 2;

                formatted.push_str(&mac[start..end]);

                if i != 5 {
                    formatted.push(':');
                }
            }

            Some(Mac(formatted))
        } else {
            None
        }
    }
}
