pub mod commands;

use chrono::{DateTime, Utc};

use self::commands::RegisterDeviceGroup;

#[derive(Default)]
pub struct DeviceGroupAggregate {
    pub id: i64,

    // defacto primary key
    pub serial_number: String,
    pub created_at: DateTime<Utc>,
}

impl DeviceGroupAggregate {
    pub(crate) fn new(cmd: RegisterDeviceGroup) -> Self {
        // Id could be created at the backend or by snowflake if global identifier is required.
        Self {
            serial_number: cmd.device_group_serial,
            created_at: Utc::now(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test_device_group {
    use chrono::{DateTime, Utc};

    use super::{commands::RegisterDeviceGroup, DeviceGroupAggregate};

    // Infallible operation which won't return error.
    #[test]
    fn create_device_group() {
        //GIVEN
        let cmd = RegisterDeviceGroup {
            device_group_serial: "A1".to_string(),
        };
        //WHEN

        let group = DeviceGroupAggregate::new(cmd);

        //THEN
        assert!(!group.serial_number.is_empty());

        assert_ne!(group.created_at, DateTime::<Utc>::default());
    }
}
