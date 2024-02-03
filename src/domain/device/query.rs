use chrono::{DateTime, Utc};

pub struct GetDeviceAverageTemperatureDuringPeriodQuery {
    pub serial_number: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

pub struct GetDeviceGroupAverageTemperatureDuringPeriodQuery {
    pub device_group_serial: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
