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
        Ok(self.repo.update(aggregate).await?.into())
    }
}

#[cfg(test)]
mod test_device_handler {
    use chrono::{Duration, Utc};

    use crate::{
        adapters::database::mock_db::MockDb,
        domain::{
            device::commands::{RegisterDevice, SaveDeviceTemperature},
            response::Error,
        },
        services::handlers::{
            device_group::test_device_handler::group_creating_helper, RepositoryHandler,
        },
    };

    // Without existing device group, it will error out saying "Not Found"
    #[tokio::test]
    async fn test_register_device_unhappy_case() {
        //GIVEN
        let db = MockDb;

        //WHEN
        let cmd = RegisterDevice {
            serial_number: "C48302DDL".to_string(),
            device_group_serial: "A6".to_string(),
        };
        let handler = RepositoryHandler::new(cmd, db.clone());
        let res = handler.handle().await;

        //THEN
        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), Error::NotFound));
    }

    #[tokio::test]
    async fn test_register_device_happy_case() {
        use crate::domain::device::repository::TDeviceQuery;
        //GIVEN
        // precondition: creation of group
        group_creating_helper("B1").await;
        let db = MockDb;

        //WHEN
        let cmd = RegisterDevice {
            serial_number: "C8302DDF".to_string(),
            device_group_serial: "B1".to_string(),
        };
        let handler = RepositoryHandler::new(cmd, db.clone());
        handler.handle().await.unwrap();

        //THEN
        let aggregate = db.get("C8302DDF").await.unwrap();
        assert_eq!(aggregate.serial_number, "C8302DDF".to_string());
    }

    #[tokio::test]
    async fn test_save_temperatures() {
        use crate::domain::device::repository::TDeviceQuery;
        //GIVEN
        // precondition: creation of group and device
        group_creating_helper("A3").await;

        let db = MockDb;
        let cmd = RegisterDevice {
            serial_number: "C48302DDK".to_string(),
            device_group_serial: "A3".to_string(),
        };
        let handler = RepositoryHandler::new(cmd, db.clone());
        handler.handle().await.unwrap();

        //WHEN
        let cmd = SaveDeviceTemperature {
            serial_number: "C48302DDK".to_string(),
            interval: 300,
            temperatures: "FFFE00010003FFFE00010003FFFE00010003FFFE00010003".to_string(),
            registered_at: Utc::now() - Duration::minutes(10),
        };
        let handler = RepositoryHandler::new(cmd, db.clone());
        handler.handle().await.unwrap();

        //THEN
        let aggregate = db.get("C48302DDK").await.unwrap();
        assert_eq!(aggregate.temperatures.len(), 12);
    }
}
