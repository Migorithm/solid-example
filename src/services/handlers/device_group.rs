use crate::domain::{
    device_group::{
        commands::RegisterDeviceGroup, repository::TDeviceGroupPersist, DeviceGroupAggregate,
    },
    response::{Error, Response},
};

use super::RepositoryHandler;

impl<R> RepositoryHandler<RegisterDeviceGroup, R>
where
    R: TDeviceGroupPersist,
{
    pub async fn handle(self) -> Result<Response, Error> {
        let aggregate = DeviceGroupAggregate::new(self.command);
        Ok(self.repo.add(aggregate).await?.into())
    }
}
