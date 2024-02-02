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

    // ? How are you going to make sure of idempotency
    pub fn save_temperatures(&mut self, cmd: SaveDeviceTemperature) -> Result<(), Error> {
        // To prevent frequent allocation
        self.temperatures =
            Vec::with_capacity(f64::ceil(cmd.temperatures.len() as f64 / 4.0) as usize);

        let mut from = 0;
        while from < cmd.temperatures.len() {
            let chunk: String = cmd
                .temperatures
                .chars()
                .into_iter()
                .skip(from)
                .take(4)
                .collect();

            self.temperatures.push(DeviceTemperature::new(
                self.device_id,
                cmd.registered_at,
                &chunk.to_string(),
            )?);
            from += 4;
        }
        Ok(())
    }
}

#[derive(Clone)]
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
        device.save_temperatures(cmd).unwrap();

        //THEN
        assert!(!device.temperatures.is_empty());
        assert_eq!(
            device
                .temperatures
                .into_iter()
                .map(|t| t.temperature)
                .collect::<Vec<_>>(),
            vec![-2, 1, 3, -2, 1, 3, -2, 1, 3, -2, 1, 3]
        );
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
