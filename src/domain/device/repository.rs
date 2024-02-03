use crate::domain::{device_group::DeviceGroupAggregate, error::Error};

use super::DeviceAggregate;

pub trait TDeviceQuery {
    fn get(
        &self,
        serial_number: &str,
    ) -> impl std::future::Future<Output = Result<DeviceAggregate, Error>> + Send;

    fn list_by_group(
        &self,
        device_group_serial_number: &str,
    ) -> impl std::future::Future<Output = Result<Vec<DeviceAggregate>, Error>> + Send;
}

pub trait TDevicePersist {
    fn add(
        &self,
        device: DeviceAggregate,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

// For the following domain to work, it requires to query against device group
pub trait TDeviceGroupQuery {
    fn get(
        &self,
        device_group_serial: &str,
    ) -> impl std::future::Future<Output = Result<DeviceGroupAggregate, Error>> + Send;
}
