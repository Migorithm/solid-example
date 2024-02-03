use serde::Deserialize;

use crate::domain::device::commands::SaveDeviceTemperature;
use crate::domain::response::Error;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

#[derive(Deserialize)]
pub struct SaveDeviceTemperatureBody {
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    pub interval: i64,
    pub temperatures: String,
    pub registered_at: String,
}

impl SaveDeviceTemperatureBody {
    pub fn into_command(self) -> Result<SaveDeviceTemperature, Error> {
        let naive_time = NaiveDateTime::parse_from_str(&self.registered_at, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| Error::SchemaError)?;

        let registered_at: DateTime<Utc> = Utc.from_local_datetime(&naive_time).unwrap();

        Ok(SaveDeviceTemperature {
            serial_number: self.serial_number,
            interval: self.interval,
            temperatures: self.temperatures,
            registered_at,
        })
    }
}
