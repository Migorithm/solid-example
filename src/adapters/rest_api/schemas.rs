use serde::Deserialize;

use crate::domain::device::commands::SaveDeviceTemperature;
use crate::domain::response::Error;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

pub mod in_schema {
    use crate::domain::device::query::{
        GetDeviceAverageTemperatureDuringPeriodQuery,
        GetDeviceGroupAverageTemperatureDuringPeriodQuery,
    };

    use super::*;
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
            Ok(SaveDeviceTemperature {
                serial_number: self.serial_number,
                interval: self.interval,
                temperatures: self.temperatures,
                registered_at: convert_string_to_utc_datetime(&self.registered_at)?,
            })
        }
    }

    #[derive(Deserialize)]
    pub struct GetDeviceAverageTemperatureDuringPeriod {
        #[serde(rename = "serialNumber")]
        pub serial_number: String,
        #[serde(rename = "startDate")]
        pub start_date: String,
        #[serde(rename = "endDate")]
        pub end_date: String,
    }
    impl GetDeviceAverageTemperatureDuringPeriod {
        pub fn into_query(self) -> Result<GetDeviceAverageTemperatureDuringPeriodQuery, Error> {
            let start_date = convert_string_to_utc_datetime(&self.start_date)?;
            let end_date = convert_string_to_utc_datetime(&self.end_date)?;

            Ok(GetDeviceAverageTemperatureDuringPeriodQuery {
                serial_number: self.serial_number,
                start_date,
                end_date,
            })
        }
    }

    #[derive(Deserialize)]
    pub struct GetDeviceGroupAverageTemperatureDuringPeriod {
        #[serde(rename = "deviceGroupSerial")]
        pub device_group_serial: String,
        #[serde(rename = "startDate")]
        pub start_date: String,
        #[serde(rename = "endDate")]
        pub end_date: String,
    }
    impl GetDeviceGroupAverageTemperatureDuringPeriod {
        pub fn into_query(
            self,
        ) -> Result<GetDeviceGroupAverageTemperatureDuringPeriodQuery, Error> {
            let start_date = convert_string_to_utc_datetime(&self.start_date)?;
            let end_date = convert_string_to_utc_datetime(&self.end_date)?;

            Ok(GetDeviceGroupAverageTemperatureDuringPeriodQuery {
                device_group_serial: self.device_group_serial,
                start_date,
                end_date,
            })
        }
    }
}

pub mod out_schema {
    use serde::Serialize;

    use crate::domain::{device::DeviceAggregate, device_group::DeviceGroupAggregate};

    #[derive(Serialize)]
    pub struct CommonOutSchema<T: Serialize> {
        msg: String,
        data: T,
    }

    #[derive(Serialize)]
    pub struct DeviceGroupOut {
        #[serde(rename = "deviceId")]
        pub device_id: i64,
        #[serde(rename = "serialNumber")]
        pub serial_number: String,

        #[serde(rename = "deviceGroup")]
        pub device_group: DeviceGroupAggregate,
        #[serde(rename = "createdAt")]
        pub created_at: String,
    }

    impl From<(DeviceAggregate, DeviceGroupAggregate)> for CommonOutSchema<DeviceGroupOut> {
        fn from(value: (DeviceAggregate, DeviceGroupAggregate)) -> Self {
            Self {
                msg: "success".to_string(),
                data: DeviceGroupOut {
                    device_id: value.0.device_id,
                    serial_number: value.0.serial_number,
                    created_at: value.0.created_at.to_string(),
                    device_group: value.1,
                },
            }
        }
    }

    impl From<DeviceGroupAggregate> for CommonOutSchema<DeviceGroupAggregate> {
        fn from(value: DeviceGroupAggregate) -> Self {
            Self {
                msg: "success".to_string(),
                data: value,
            }
        }
    }

    #[derive(Serialize)]
    pub struct DeviceWithAverageTemperatureDuringPeriod {
        pub id: i64,
        #[serde(rename = "serialNumber")]
        pub serial_number: String,

        #[serde(rename = "averageTemperature")]
        pub average_temperature: f32,
    }
    impl From<(DeviceAggregate, f32)> for DeviceWithAverageTemperatureDuringPeriod {
        fn from(value: (DeviceAggregate, f32)) -> Self {
            Self {
                id: value.0.device_id,
                serial_number: value.0.serial_number,
                average_temperature: value.1,
            }
        }
    }
    impl From<DeviceWithAverageTemperatureDuringPeriod>
        for CommonOutSchema<DeviceWithAverageTemperatureDuringPeriod>
    {
        fn from(value: DeviceWithAverageTemperatureDuringPeriod) -> Self {
            Self {
                msg: "success".to_string(),
                data: value,
            }
        }
    }

    impl From<Vec<DeviceWithAverageTemperatureDuringPeriod>>
        for CommonOutSchema<Vec<DeviceWithAverageTemperatureDuringPeriod>>
    {
        fn from(value: Vec<DeviceWithAverageTemperatureDuringPeriod>) -> Self {
            Self {
                msg: "success".to_string(),
                data: value,
            }
        }
    }
}

fn convert_string_to_utc_datetime(given: &str) -> Result<DateTime<Utc>, Error> {
    let naive_time = NaiveDateTime::parse_from_str(given, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| Error::SchemaError)?;
    Ok(Utc.from_local_datetime(&naive_time).unwrap())
}

#[test]
fn test_naive_time_conversion() {
    let naive = "2023-02-01 19:00:00";
    convert_string_to_utc_datetime(naive).unwrap();
}

#[test]
fn datetime_conversion() {
    use chrono::DateTime;
    use chrono::NaiveDateTime;
    use chrono::TimeZone;
    use chrono::Utc;
    let date_str = "2023-01-01 16:00:00";

    // Parse the string using the specified format
    let datetime = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse datetime");

    let _date_time: DateTime<Utc> = Utc.from_local_datetime(&datetime).unwrap();
}
