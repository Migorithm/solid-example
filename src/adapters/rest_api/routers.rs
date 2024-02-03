use axum::{
    extract::Query,
    routing::{get, post},
    Json, Router,
};

use crate::{
    adapters::{
        database::mock_db::MockDb,
        rest_api::response::{Exception, WebResponse},
    },
    domain::{
        device::commands::RegisterDevice,
        device_group::{commands::RegisterDeviceGroup, DeviceGroupAggregate},
        response::Error,
        response::Response,
    },
    services::handlers::{CommandHandler, QueryHandler},
};

use super::schemas::{
    in_schema::{
        GetDeviceAverageTemperatureDuringPeriod, GetDeviceGroupAverageTemperatureDuringPeriod,
        SaveDeviceTemperatureBody,
    },
    out_schema::{CommonOutSchema, DeviceGroupOut, DeviceWithAverageTemperatureDuringPeriod},
};

pub async fn register_device(
    Json(cmd): Json<RegisterDevice>,
) -> Result<WebResponse<CommonOutSchema<DeviceGroupOut>>, Exception<Error>> {
    let out = CommandHandler::new(cmd, MockDb).handle().await?.into();

    Ok(WebResponse(out))
}

pub async fn register_device_group(
    Json(cmd): Json<RegisterDeviceGroup>,
) -> Result<WebResponse<CommonOutSchema<DeviceGroupAggregate>>, Exception<Error>> {
    let res = CommandHandler::new(cmd, MockDb).handle().await?;

    Ok(WebResponse(res.into()))
}

pub async fn save_device_temperature(
    Json(cmd): Json<SaveDeviceTemperatureBody>,
) -> Result<WebResponse<Response>, Exception<Error>> {
    let res = CommandHandler::new(cmd.into_command()?, MockDb)
        .handle()
        .await?;

    Ok(WebResponse(res))
}

pub async fn get_device_average_tempature_during_period(
    Query(query): Query<GetDeviceAverageTemperatureDuringPeriod>,
) -> Result<WebResponse<CommonOutSchema<DeviceWithAverageTemperatureDuringPeriod>>, Exception<Error>>
{
    let query = query.into_query()?;
    let res: DeviceWithAverageTemperatureDuringPeriod =
        QueryHandler::new(query, MockDb).handle().await?.into();

    Ok(WebResponse(res.into()))
}

pub async fn get_device_group_average_tempature_during_period(
    Query(query): Query<GetDeviceGroupAverageTemperatureDuringPeriod>,
) -> Result<
    WebResponse<CommonOutSchema<Vec<DeviceWithAverageTemperatureDuringPeriod>>>,
    Exception<Error>,
> {
    let query = query.into_query()?;
    let res: Vec<DeviceWithAverageTemperatureDuringPeriod> = QueryHandler::new(query, MockDb)
        .handle()
        .await?
        .into_iter()
        .map(
            |(device, average)| DeviceWithAverageTemperatureDuringPeriod {
                id: device.device_id,
                serial_number: device.serial_number,
                average_temperature: average,
            },
        )
        .collect::<Vec<_>>();

    Ok(WebResponse(res.into()))
}

pub fn routers() -> Router {
    Router::new()
        .route("/device_groups", post(register_device_group))
        .route(
            "/device_groups/temperature",
            get(get_device_group_average_tempature_during_period),
        )
        .route(
            "/devices",
            post(register_device).patch(save_device_temperature),
        )
        .route(
            "/devices/temperature",
            get(get_device_average_tempature_during_period),
        )
}
