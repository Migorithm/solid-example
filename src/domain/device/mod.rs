pub mod commands;
pub mod query;
pub mod repository;
use crate::domain::response::Error;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use serde::Serialize;

use self::commands::RegisterDevice;
use self::commands::SaveDeviceTemperature;

#[derive(Default, Clone, Serialize, Debug)]
pub struct DeviceAggregate {
    #[serde(rename = "deviceId")]
    pub device_id: i64,
    #[serde(rename = "deviceGroupSerialNumber")]
    pub device_group_serial_number: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(skip_serializing)]
    pub temperatures: Vec<DeviceTemperature>,
}

impl DeviceAggregate {
    pub fn new(cmd: RegisterDevice) -> Self {
        Self {
            device_group_serial_number: cmd.device_group_serial,
            serial_number: cmd.serial_number,
            created_at: Utc::now(),
            ..Default::default()
        }
    }

    // ? How are you going to make sure of idempotency
    pub fn save_temperatures(&mut self, cmd: SaveDeviceTemperature) -> Result<(), Error> {
        // To prevent frequent allocation
        let mut temperatures =
            Vec::with_capacity(f64::ceil(cmd.temperatures.len() as f64 / 4.0) as usize);

        let mut loop_cnt = 0;
        while loop_cnt * 4 < cmd.temperatures.len() {
            let chunk: String = cmd
                .temperatures
                .chars()
                .skip(loop_cnt * 4)
                .take(4)
                .collect();
            temperatures.push(DeviceTemperature::new(
                self.device_id,
                cmd.registered_at + Duration::seconds(cmd.interval) * loop_cnt as i32,
                &chunk.to_string(),
            )?);
            loop_cnt += 1;
        }
        self.temperatures.extend(temperatures);
        Ok(())
    }

    pub fn get_average_temperature_during_period(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> f32 {
        let temperature_in_range = self
            .temperatures
            .iter()
            .filter(|temp| start_date <= temp.checked_at && temp.checked_at <= end_date)
            .collect::<Vec<_>>();

        let average: f32 = temperature_in_range
            .iter()
            .map(|f| f.temperature as f32)
            .sum::<f32>()
            / temperature_in_range.len() as f32;

        average
    }
}

#[derive(Clone, Debug)]
pub struct DeviceTemperature {
    pub device_id: i64,
    pub temperature: i16,
    pub checked_at: DateTime<Utc>,
}

impl DeviceTemperature {
    pub fn new(
        device_id: i64,
        checked_at: DateTime<Utc>,
        tempature_in_hex: &str,
    ) -> Result<Self, Error> {
        Ok(Self {
            device_id,
            checked_at,
            temperature: Self::hex_to_decimal(tempature_in_hex)?,
        })
    }

    pub(crate) fn hex_to_decimal(hex: &str) -> Result<i16, Error> {
        if hex.len() != 4 {
            eprintln!("[ERROR] Wrong length given ");
            return Err(Error::ConversionFailed);
        }

        Ok(i32::from_str_radix(hex, 16).map_err(|err| {
            eprintln!("[ERROR] Conversion failed {}", err);
            Error::ConversionFailed
        })? as i16)
    }
}

#[cfg(test)]
mod test_device {
    use chrono::{DateTime, Duration, Utc};

    use crate::domain::device::{DeviceAggregate, DeviceTemperature};

    use super::commands::{RegisterDevice, SaveDeviceTemperature};

    // Infallible operation which won't return error.
    #[test]
    fn create_device_group() {
        //GIVEN
        let cmd = RegisterDevice {
            serial_number: "C48302DDL".to_string(),
            device_group_serial: "A1".to_string(),
        };
        //WHEN

        let device = DeviceAggregate::new(cmd);

        //THEN
        assert!(!device.serial_number.is_empty());

        assert_ne!(device.created_at, DateTime::<Utc>::default());

        assert_ne!(device.created_at, DateTime::<Utc>::default());
    }

    #[test]
    fn save_temperatures() {
        //GIVEN

        let cmd = RegisterDevice {
            serial_number: "C48302DDL".to_string(),
            device_group_serial: "A1".to_string(),
        };

        let mut device = DeviceAggregate::new(cmd);

        //WHEN
        let registered_at = Utc::now() - Duration::minutes(5);
        let interval = 300;
        let cmd = SaveDeviceTemperature {
            serial_number: "C48302DDL".to_string(),
            interval,
            temperatures: "FFFE00010003FFFE00010003FFFE00010003FFFE00010003".to_string(),
            registered_at,
        };
        device.save_temperatures(cmd).unwrap();

        //THEN
        assert!(!device.temperatures.is_empty());
        assert_eq!(
            device
                .temperatures
                .iter()
                .map(|t| t.temperature)
                .collect::<Vec<_>>(),
            vec![-2, 1, 3, -2, 1, 3, -2, 1, 3, -2, 1, 3]
        );
        assert_eq!(
            device
                .temperatures
                .iter()
                .map(|t| t.checked_at)
                .collect::<Vec<_>>(),
            vec![
                registered_at,
                registered_at + Duration::seconds(interval),
                registered_at + Duration::seconds(interval) * 2,
                registered_at + Duration::seconds(interval) * 3,
                registered_at + Duration::seconds(interval) * 4,
                registered_at + Duration::seconds(interval) * 5,
                registered_at + Duration::seconds(interval) * 6,
                registered_at + Duration::seconds(interval) * 7,
                registered_at + Duration::seconds(interval) * 8,
                registered_at + Duration::seconds(interval) * 9,
                registered_at + Duration::seconds(interval) * 10,
                registered_at + Duration::seconds(interval) * 11
            ]
        )
    }

    #[test]
    fn tempeature_conversion() {
        //GIVEN
        // the following should become -2,1,3,-2,1,3,-2,1,3 ...
        let hex_temperature = ["FFFE", "0001", "0003"];

        //WHEN
        let test_vec = hex_temperature
            .into_iter()
            .map(|ele| DeviceTemperature::hex_to_decimal(ele).unwrap())
            .collect::<Vec<_>>();

        //THEN
        assert_eq!(test_vec, vec![-2, 1, 3]);
    }
}
