use super::mock_db::{device_group_table, device_table, MockDb, AUTOINCREMENTED_VALUE_FOR_DEVICE};
use crate::{
    adapters::database::mock_db::AUTOINCREMENTED_VALUE_FOR_DEVICE_GROUP,
    domain::{
        device::{
            repository::{TDeviceGroupQuery, TDevicePersist, TDeviceQuery},
            DeviceAggregate,
        },
        device_group::{repository::TDeviceGroupPersist, DeviceGroupAggregate},
        response::Error,
    },
};
/// Although preferrable to separate Repository per aggregate, I lumped all of them together for
/// simplicity reason.
use std::sync::atomic::Ordering;

impl TDeviceGroupPersist for MockDb {
    async fn add(&self, mut group: DeviceGroupAggregate) -> Result<(), Error> {
        group.device_group_id =
            AUTOINCREMENTED_VALUE_FOR_DEVICE_GROUP.fetch_add(1, Ordering::SeqCst);

        if device_group_table()
            .write()
            .await
            .iter()
            .find(|existing| existing.serial_number == group.serial_number)
            .is_some()
        {
            println!("given serial already exist {}", group.serial_number);
            return Err(Error::DuplicateKeyError);
        };
        device_group_table().write().await.push(group);
        Ok(())
    }
}

impl TDeviceGroupQuery for MockDb {
    async fn get(&self, device_group_serial: &str) -> Result<DeviceGroupAggregate, Error> {
        Ok(device_group_table()
            .read()
            .await
            .iter()
            .find(|group| group.serial_number == device_group_serial)
            .ok_or(Error::NotFound)?
            .clone())
    }
}

impl TDevicePersist for MockDb {
    async fn add(&self, mut device: DeviceAggregate) -> Result<(), Error> {
        device.device_id = AUTOINCREMENTED_VALUE_FOR_DEVICE.fetch_add(1, Ordering::SeqCst);

        if device_table()
            .write()
            .await
            .iter()
            .find(|existing| existing.serial_number == device.serial_number)
            .is_some()
        {
            return Err(Error::DuplicateKeyError);
        };
        device_table().write().await.push(device);
        Ok(())
    }
    async fn update(&self, device: DeviceAggregate) -> Result<(), Error> {
        *device_table()
            .write()
            .await
            .iter_mut()
            .find(|device| device.serial_number == device.serial_number)
            .ok_or(Error::NotFound)? = device;
        Ok(())
    }
}

impl TDeviceQuery for MockDb {
    async fn get(&self, serial_number: &str) -> Result<DeviceAggregate, Error> {
        Ok(device_table()
            .read()
            .await
            .iter()
            .find(|device| device.serial_number == serial_number)
            .ok_or(Error::NotFound)?
            .clone())
    }

    async fn list_by_group(
        &self,
        device_group_serial_number: &str,
    ) -> Result<Vec<DeviceAggregate>, Error> {
        Ok(device_table()
            .read()
            .await
            .iter()
            .filter(|device| &device.device_group_serial_number == device_group_serial_number)
            .cloned()
            .collect())
    }
}
