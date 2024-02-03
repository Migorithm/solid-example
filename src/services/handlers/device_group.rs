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

#[cfg(test)]
pub mod test_device_handler {
    use chrono::{DateTime, Utc};

    use crate::{
        adapters::database::mock_db::MockDb,
        domain::{
            device::repository::TDeviceGroupQuery, device_group::commands::RegisterDeviceGroup,
        },
        services::handlers::RepositoryHandler,
    };

    #[tokio::test]
    async fn test_register_device_group() {
        //GIVEN
        let db = MockDb;

        //WHEN
        let cmd = RegisterDeviceGroup {
            device_group_serial: "A11".to_string(),
        };
        let handler = RepositoryHandler::new(cmd, db.clone());
        handler.handle().await.unwrap();

        //THEN
        let inserted_group = db.get("A11").await.unwrap();

        assert_eq!(inserted_group.serial_number, "A11".to_string());

        assert_ne!(inserted_group.created_at, DateTime::<Utc>::default());
    }

    pub async fn group_creating_helper(serial: &str) {
        let db = MockDb;
        let cmd = RegisterDeviceGroup {
            device_group_serial: serial.to_string(),
        };
        let handler = RepositoryHandler::new(cmd, db.clone());
        //WHEN
        handler.handle().await.unwrap();
    }
}
