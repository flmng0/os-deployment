use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;
use wmi::*;

use osd_core::Mac;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_NetworkAdapter")]
#[serde(rename_all = "PascalCase")]
pub struct NetworkAdapter {
    mac_address: String,
}

#[derive(Error, Debug)]
pub enum GetMacError {
    #[error("connecting to WMI failed: {0}")]
    WmiConnectionInit(WMIError),
    #[error("running WMI query failed")]
    Query,
    #[error("no entries returned from WMI")]
    NoResults,
}

pub fn get_local_mac_address() -> Result<Mac, GetMacError> {
    let com_lib = unsafe { COMLibrary::assume_initialized() };
    let wmi_con = WMIConnection::new(com_lib).map_err(|e| GetMacError::WmiConnectionInit(e))?;

    let mut filters = HashMap::new();
    filters.insert("NetConnectionStatus".to_owned(), FilterValue::Number(2));

    let results: Vec<NetworkAdapter> = wmi_con
        .filtered_query::<NetworkAdapter>(&filters)
        .map_err(|_| GetMacError::Query)?;

    results
        .get(0)
        .and_then(|entry| Mac::new(&entry.mac_address))
        .ok_or_else(|| GetMacError::NoResults)
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_SystemEnclosure")]
#[serde(rename_all = "PascalCase")]
pub struct SystemEnclosure {
    chassis_types: Option<Vec<u16>>,
    serial_number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_ComputerSystem")]
#[serde(rename_all = "PascalCase")]
pub struct ComputerSystem {
    manufacturer: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_BIOS")]
#[serde(rename_all = "PascalCase")]
pub struct Bios {
    serial_number: String,
}

fn get_serial_number(wmi_con: &WMIConnection, enclosure: &SystemEnclosure) -> Option<String> {
    let invalid = match &enclosure.serial_number {
        Some(serial) => {
            serial.is_empty()
                || ["NONE", "00000000"].contains(&serial.as_str())
                || serial.contains("OEM")
        }
        None => true,
    };

    if !invalid {
        return enclosure.serial_number.clone();
    }

    let results: Vec<Bios> = wmi_con.query().unwrap_or_default();

    results.first().map(|r| r.serial_number.clone())
}

pub fn generate_asset_id() -> Option<String> {
    let com_lib = unsafe { COMLibrary::assume_initialized() };
    let wmi_con = WMIConnection::new(com_lib).ok()?;

    let enclosure: SystemEnclosure = wmi_con
        .get_by_path(r#"\\.\root\cimv2:Win32_SystemEnclosure.Tag='System Enclosure 0'"#)
        .ok()?;

    // Get manufacturer

    let manufacturer: Option<String> = wmi_con
        .query::<ComputerSystem>()
        .ok()?
        .iter()
        .filter_map(|sys| sys.manufacturer.clone())
        .filter(|m| !m.is_empty())
        .next();

    // Get serial number
    let serial = get_serial_number(&wmi_con, &enclosure)?;

    // Get chassis type (desktop or laptop)
    let device_type = get_device_type(&enclosure);

    // Put together like <chassis type>-<manufacturer>-<first 6 of serial number>

    let chassis_part = match device_type {
        DeviceType::Unknown | DeviceType::Laptop => "LT",
        DeviceType::Desktop => "PC",
    };

    let manufacturer_part = manufacturer.map(|m| m.chars().next().unwrap());

    let serial_len = serial.len();
    let serial_part = serial[(serial_len - 8)..].to_string();

    let asset_id = match manufacturer_part {
        Some(manufacturer_part) => format!("{chassis_part}-{manufacturer_part}-{serial_part}"),
        None => format!("{chassis_part}-{serial_part}"),
    };

    Some(asset_id)
}

enum DeviceType {
    Desktop,
    Laptop,
    Unknown,
}

impl DeviceType {
    fn from_chassis(id: &u16) -> Self {
        match id {
            1 | 2 => Self::Unknown,
            3..=7 | 13 | 15..=24 => Self::Desktop,
            8..=12 | 14 | 30..=32 => Self::Laptop,
            _ => Self::Unknown,
        }
    }
}

fn get_device_type(enclosure: &SystemEnclosure) -> DeviceType {
    match &enclosure.chassis_types {
        None => DeviceType::Unknown,
        Some(ids) => ids
            .iter()
            .map(|id| DeviceType::from_chassis(id))
            .filter(|dtype| !matches!(dtype, DeviceType::Unknown))
            .next()
            .unwrap_or(DeviceType::Unknown),
    }
}
