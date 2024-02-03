use crate::domain::{
    device::{
        commands::{RegisterDevice, SaveDeviceTemperature},
        repository::{TDeviceGroupQuery, TDevicePersist, TDeviceQuery},
        DeviceAggregate,
    },
    response::{Error, Response},
};

use super::RepositoryHandler;

//TODO return type matching according to requirement
impl<R> RepositoryHandler<RegisterDevice, R>
where
    R: TDevicePersist + TDeviceGroupQuery,
{
    pub async fn handle(self) -> Result<Response, Error> {
        // Validate if group actually exists
        self.repo.get(&self.command.device_group_serial).await?;

        let aggregate = DeviceAggregate::new(self.command);
        Ok(self.repo.add(aggregate).await?.into())
    }
}

//TODO return type matching according to requirement
impl<R> RepositoryHandler<SaveDeviceTemperature, R>
where
    R: TDevicePersist + TDeviceQuery,
{
    pub async fn handle(self) -> Result<Response, Error> {
        let mut aggregate = self.repo.get(&self.command.serial_number).await?;
        aggregate.save_temperatures(self.command)?;
        Ok(self.repo.add(aggregate).await?.into())
    }
}

#[cfg(test)]
mod test_device_handler {
    use crate::{
        adapters::database::mock_db::MockDb, domain::device::commands::RegisterDevice,
        services::handlers::RepositoryHandler,
    };

    #[tokio::test]
    async fn test_register_device() {
        use crate::domain::device::repository::TDeviceQuery;
        //GIVEN
        let db = MockDb;
        let cmd = RegisterDevice {
            serial_number: "C48302DDL".to_string(),
            device_group_serial: "A1".to_string(),
        };
        let handler = RepositoryHandler::new(cmd, db.clone());

        //WHEN
        handler.handle().await.unwrap();

        //THEN
        let aggregate = db.get("C48302DDL").await.unwrap();
        assert_eq!(aggregate.serial_number, "C48302DDL".to_string());
    }
}
