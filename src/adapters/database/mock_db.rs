use std::sync::{atomic::AtomicI64, Arc, OnceLock};

use tokio::sync::RwLock;

use crate::domain::{device::DeviceAggregate, device_group::DeviceGroupAggregate};

#[derive(Clone)]
pub struct MockDb;

//Mock table for device which `MockDb` will access
pub fn device_table() -> &'static Arc<RwLock<Vec<DeviceAggregate>>> {
    static DEVICE_TABLE: OnceLock<Arc<RwLock<Vec<DeviceAggregate>>>> = OnceLock::new();
    DEVICE_TABLE.get_or_init(|| Arc::new(RwLock::new(vec![])))
}

//Mock table for device group which `MockDb` will access
pub fn device_group_table() -> &'static Arc<RwLock<Vec<DeviceGroupAggregate>>> {
    static DEVICE_TABLE: OnceLock<Arc<RwLock<Vec<DeviceGroupAggregate>>>> = OnceLock::new();
    DEVICE_TABLE.get_or_init(|| Arc::new(RwLock::new(vec![])))
}

pub static AUTOINCREMENTED_VALUE_FOR_DEVICE: AtomicI64 = AtomicI64::new(0);
pub static AUTOINCREMENTED_VALUE_FOR_DEVICE_GROUP: AtomicI64 = AtomicI64::new(0);
