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
    net_connection_status: u16,
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
    serial_number: String,
}

#[derive(Deserialize, Debug)]
#[seride(rename = "Win32_ComputerSystem")]
#[serde(rename_all = "PascalCase")]
pub struct ComputerSystem {
    manufacturer: String,
}

pub fn generate_asset_id() -> Option<String> {
    let com_lib = unsafe { COMLibrary::assume_initialized() };
    let wmi_con = WMIConnection::new(com_lib).ok()?;

    // Get manufacturer

    // Get serial number

    // Get chassis type (desktop or laptop)

    // Put together like <manufacturer>-<chassis type>-<first 6 of serial number>

    let mut filters = HashMap::new();
}
