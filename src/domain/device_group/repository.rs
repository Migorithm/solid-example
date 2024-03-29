use crate::domain::response::Error;

use super::DeviceGroupAggregate;

pub trait TDeviceGroupPersist {
    fn add(
        &self,
        device: &mut DeviceGroupAggregate,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
