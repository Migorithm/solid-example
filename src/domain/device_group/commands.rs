use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterDeviceGroup {
    #[serde(rename = "deviceGroupSerial")]
    pub device_group_serial: String,
}
