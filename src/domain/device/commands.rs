use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterDevice {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "deviceGroupSerial")]
    pub device_group_serial: String,
}

#[derive(Deserialize)]
pub struct SaveDeviceTemperature {
    pub serial_number: String,
    pub interval: i64,
    pub temperatures: String,
    pub registered_at: DateTime<Utc>,
}
