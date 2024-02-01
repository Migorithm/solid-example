pub mod commands;
use chrono::DateTime;
use chrono::Utc;

use crate::domain::error::Error;

use self::commands::RegisterDevice;
use self::commands::SaveDeviceTemperature;

#[derive(Default)]
pub struct DeviceAggregate {
    pub device_id: i64,
    pub device_group_id: i64,
    pub serial_number: String,
    pub created_at: DateTime<Utc>,

    // child entity
    pub temperatures: Vec<DeviceTemperature>,
}

impl DeviceAggregate {
    pub fn new(cmd: RegisterDevice, device_group_id: i64) -> Self {
        Self {
            device_group_id,
            serial_number: cmd.serial_number,
            created_at: Utc::now(),
            ..Default::default()
        }
    }

    pub fn save_temperatures(&mut self, cmd: SaveDeviceTemperature) {
        todo!()
    }
}

pub struct DeviceTemperature {
    pub device_id: i64,
    pub temperature: i16,
    pub checked_at: DateTime<Utc>,
}

impl DeviceTemperature {
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
        let group_id = 0;
        //WHEN

        let device = DeviceAggregate::new(cmd, group_id);

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
        let group_id = 0;

        let mut device = DeviceAggregate::new(cmd, group_id);

        //WHEN
        let cmd = SaveDeviceTemperature {
            serial_number: "C48302DDL".to_string(),
            interval: 300,
            temperatures: "FFFE00010003FFFE00010003FFFE00010003FFFE00010003".to_string(),
            registered_at: Utc::now() - Duration::minutes(5),
        };
        device.save_temperatures(cmd);

        //THEN
        assert!(!device.temperatures.is_empty());
        assert_eq!(device.temperatures.first().unwrap().temperature, -2);
        assert_eq!(device.temperatures.last().unwrap().temperature, 3);
    }

    #[test]
    fn tempeature_convsion() {
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
