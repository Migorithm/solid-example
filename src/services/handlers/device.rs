use crate::domain::{
    device::{
        commands::{RegisterDevice, SaveDeviceTemperature},
        query::{
            GetDeviceAverageTemperatureDuringPeriodQuery,
            GetDeviceGroupAverageTemperatureDuringPeriodQuery,
        },
        repository::{TDeviceGroupQuery, TDevicePersist, TDeviceQuery},
        DeviceAggregate,
    },
    device_group::DeviceGroupAggregate,
    response::{Error, Response},
};

use super::{CommandHandler, QueryHandler};

impl<R> CommandHandler<RegisterDevice, R>
where
    R: TDevicePersist + TDeviceGroupQuery,
{
    pub async fn handle(self) -> Result<(DeviceAggregate, DeviceGroupAggregate), Error> {
        // Validate if group actually exists
        let group = self.repo.get(&self.command.device_group_serial).await?;

        let mut aggregate = DeviceAggregate::new(self.command);
        self.repo.add(&mut aggregate).await?;
        Ok((aggregate, group))
    }
}

impl<R> CommandHandler<SaveDeviceTemperature, R>
where
    R: TDevicePersist + TDeviceQuery,
{
    pub async fn handle(self) -> Result<Response, Error> {
        let mut aggregate = self.repo.get(&self.command.serial_number).await?;
        aggregate.save_temperatures(self.command)?;
        Ok(self.repo.update(&mut aggregate).await?.into())
    }
}

impl<R> QueryHandler<GetDeviceAverageTemperatureDuringPeriodQuery, R>
where
    R: TDeviceQuery,
{
    pub async fn handle(self) -> Result<(DeviceAggregate, f32), Error> {
        let aggregate = self.repo.get(&self.query.serial_number).await?;
        let average = aggregate
            .get_average_temperature_during_period(self.query.start_date, self.query.end_date);

        Ok((aggregate, average))
    }
}

impl<R> QueryHandler<GetDeviceGroupAverageTemperatureDuringPeriodQuery, R>
where
    R: TDeviceQuery,
{
    pub async fn handle(self) -> Result<Vec<(DeviceAggregate, f32)>, Error> {
        let aggregates = self
            .repo
            .list_by_group(&self.query.device_group_serial)
            .await?;
        Ok(aggregates
            .into_iter()
            .map(|aggregate| {
                let average = aggregate.get_average_temperature_during_period(
                    self.query.start_date,
                    self.query.end_date,
                );
                (aggregate, average)
            })
            .collect())
    }
}

#[cfg(test)]
mod test_device_handler {
    use chrono::{Duration, Utc};

    use crate::{
        adapters::database::mock_db::MockDb,
        domain::{
            device::{
                commands::{RegisterDevice, SaveDeviceTemperature},
                query::{
                    GetDeviceAverageTemperatureDuringPeriodQuery,
                    GetDeviceGroupAverageTemperatureDuringPeriodQuery,
                },
            },
            response::Error,
        },
        services::handlers::{
            device_group::test_device_handler::group_creating_helper, CommandHandler, QueryHandler,
        },
    };

    async fn device_create_helper(device_group_serial: &str, serial_number: &str) {
        let cmd = RegisterDevice {
            serial_number: serial_number.to_string(),
            device_group_serial: device_group_serial.to_string(),
        };
        let handler = CommandHandler::new(cmd, MockDb);
        handler.handle().await.unwrap();
    }

    async fn save_temperatures_helper(serial_number: &str, temperatures: &str) {
        let db = MockDb;
        let cmd = SaveDeviceTemperature {
            serial_number: serial_number.to_string(),
            interval: 300,
            temperatures: temperatures.to_string(),
            registered_at: Utc::now(),
        };
        let handler = CommandHandler::new(cmd, db.clone());
        handler.handle().await.unwrap();
    }

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
        let handler = CommandHandler::new(cmd, db.clone());
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
        let handler = CommandHandler::new(cmd, db.clone());
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
        device_create_helper("A3", "C48302DDK").await;
        let db = MockDb;

        //WHEN
        let cmd = SaveDeviceTemperature {
            serial_number: "C48302DDK".to_string(),
            interval: 300,
            temperatures: "FFFE00010003FFFE00010003FFFE00010003FFFE00010003".to_string(),
            registered_at: Utc::now() - Duration::minutes(10),
        };
        let handler = CommandHandler::new(cmd, db.clone());
        handler.handle().await.unwrap();

        //THEN
        let aggregate = db.get("C48302DDK").await.unwrap();
        assert_eq!(aggregate.temperatures.len(), 12);
    }

    #[tokio::test]
    async fn test_get_device_average_temperature() {
        //GIVEN
        group_creating_helper("R1").await;
        device_create_helper("R1", "R48302DDK").await;
        save_temperatures_helper("R48302DDK", "FFFEFFFEFFFEFFFE").await;

        let db = MockDb;

        //WHEN
        let query = GetDeviceAverageTemperatureDuringPeriodQuery {
            serial_number: "R48302DDK".to_string(),
            start_date: Utc::now() - Duration::minutes(300),
            end_date: Utc::now() + Duration::minutes(300),
        };
        let handler = QueryHandler::new(query, db);
        let (device, average) = handler.handle().await.unwrap();
        //THEN
        assert_eq!(-2.0, average);
        assert_eq!(device.device_group_serial_number, "R1".to_string());
    }

    #[tokio::test]
    async fn test_get_device_group_average_temperature() {
        //GIVEN
        group_creating_helper("R1").await;
        device_create_helper("R1", "R18302DDK").await;
        device_create_helper("R1", "R28302DDK").await;
        save_temperatures_helper("R18302DDK", "FFFE00010003FFFE").await;
        save_temperatures_helper("R28302DDK", "FFFE000100030001").await;

        let db = MockDb;

        //WHEN
        let query = GetDeviceGroupAverageTemperatureDuringPeriodQuery {
            device_group_serial: "R1".to_string(),
            start_date: Utc::now() - Duration::minutes(300),
            end_date: Utc::now() + Duration::minutes(300),
        };
        let handler = QueryHandler::new(query, db);
        let result = handler.handle().await.unwrap();

        //THEN
        assert_eq!(result.len(), 2);
        let first_device = result.first().unwrap();
        assert_eq!(first_device.0.serial_number, "R18302DDK");
        assert_eq!(first_device.1, 0.0);

        let second_device = result.last().unwrap();
        assert_eq!(second_device.0.serial_number, "R28302DDK");
        assert_eq!(second_device.1, 0.75);
    }
}
