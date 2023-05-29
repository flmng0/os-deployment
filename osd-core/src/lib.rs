use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

pub const SERVER_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const SERVER_PORT: u16 = 4056;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    // "I'm ready to be built!"
    Advertise(DeviceInfo),
    // "Give me the current list of devices!"
    ListDevices,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    // "I've received your connection, this is your ID."
    ConnectionSuccess(u32),
    // "A user has requested to rebuild you, you can start now!"
    StartBuilding,
    // "Here is the current list of devices ready to be deployed."
    DeviceList(Vec<DeviceListing>),
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: Ipv4Addr,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeviceListing {
    info: DeviceInfo,
    added: bson::DateTime,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct DeviceInfo {
    pub hostname: Option<String>,
    pub mac_address: Mac,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
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
