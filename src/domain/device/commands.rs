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

// expected result
// { 2
//  "msg" :"success",
//  "data": {
//          "deviceId":1,
//          "serialNumber":"C48302DDL",
//          "deviceGroup" : {
//              "deviceGroupId":1,
//              "serialNumber":"A1",
//              "createdAt":"2023-01-01T00:00:00"
//          },
//          "createdAt":"2023-01-01T00:00:00"
//    }
// }

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

    let date_time: DateTime<Utc> = Utc.from_local_datetime(&datetime).unwrap();
    println!("{}", date_time);
}
