use crate::domain::{device::DeviceAggregate, device_group::DeviceGroupAggregate};

pub struct MockDb {
    pub(crate) devices: Vec<DeviceAggregate>,
    pub(crate) device_groups: Vec<DeviceGroupAggregate>,
}
