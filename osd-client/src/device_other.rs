use osd_core::Mac;

pub fn get_local_mac_address() -> Result<Mac, ()> {
    return Ok(Mac::new("AB:CD:EF:12:34:56"));
}

pub fn generate_asset_id() -> Option<String> {
    return Some("TEST_ASSET".to_string());
}
