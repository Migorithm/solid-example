use crate::domain::{
    device_group::{
        commands::RegisterDeviceGroup, repository::TDeviceGroupPersist, DeviceGroupAggregate,
    },
    error::Error,
};

use super::RepositoryHandler;

impl<R> RepositoryHandler<RegisterDeviceGroup, R>
where
    R: TDeviceGroupPersist,
{
    pub async fn handle(self) -> Result<(), Error> {
        let aggregate = DeviceGroupAggregate::new(self.command);
        self.repo.add(aggregate).await
    }
}
