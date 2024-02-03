use crate::domain::{
    device::{
        commands::{RegisterDevice, SaveDeviceTemperature},
        repository::{TDeviceGroupQuery, TDevicePersist, TDeviceQuery},
        DeviceAggregate,
    },
    error::Error,
};

use super::RepositoryHandler;

//TODO return type matching according to requirement
impl<R> RepositoryHandler<RegisterDevice, R>
where
    R: TDevicePersist + TDeviceGroupQuery,
{
    pub async fn handle(self) -> Result<(), Error> {
        let group = self.repo.get(&self.command.device_group_serial).await?;
        let aggregate = DeviceAggregate::new(self.command, group.id);
        self.repo.add(aggregate).await
    }
}

//TODO return type matching according to requirement
impl<R> RepositoryHandler<SaveDeviceTemperature, R>
where
    R: TDevicePersist + TDeviceQuery,
{
    pub async fn handle(self) -> Result<(), Error> {
        let mut aggregate = self.repo.get(&self.command.serial_number).await?;
        aggregate.save_temperatures(self.command)?;
        self.repo.add(aggregate).await
    }
}
