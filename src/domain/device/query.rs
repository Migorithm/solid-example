use chrono::{DateTime, Utc};

pub struct GetDeviceAverageTemperatureDuringPeriodQuery {
    pub serial_number: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
