use crate::domain::error::Error;

use super::DeviceGroupAggregate;

pub trait TDeviceGroupPersist {
    fn add(
        &self,
        device: DeviceGroupAggregate,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
