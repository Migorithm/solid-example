use chrono::{DateTime, Utc};

pub struct RegisterDevice {
    pub serial_number: String,
    pub device_group_serial: String,
}

pub struct SaveDeviceTemperature {
    pub serial_number: String,
    pub interval: i32,
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
