use crate::domain::{device::DeviceAggregate, device_group::DeviceGroupAggregate};

pub struct MockDb {
    devices: Vec<DeviceAggregate>,
    device_groups: Vec<DeviceGroupAggregate>,
}
