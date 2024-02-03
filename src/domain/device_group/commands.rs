use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterDeviceGroup {
    pub device_group_serial: String,
}
