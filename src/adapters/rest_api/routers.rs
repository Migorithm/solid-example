use axum::{routing::post, Json, Router};

use crate::{
    adapters::{
        database::mock_db::MockDb,
        rest_api::response::{Exception, WebResponse},
    },
    domain::{
        device::commands::RegisterDevice, device_group::commands::RegisterDeviceGroup,
        response::Error, response::Response,
    },
    services::handlers::RepositoryHandler,
};

use super::schemas::SaveDeviceTemperatureBody;

#[axum::debug_handler]
pub async fn register_device(
    Json(cmd): Json<RegisterDevice>,
) -> Result<WebResponse<Response>, Exception<Error>> {
    let res = RepositoryHandler::new(cmd, MockDb).handle().await?;

    Ok(WebResponse(res))
}

#[axum::debug_handler]
pub async fn register_device_group(
    Json(cmd): Json<RegisterDeviceGroup>,
) -> Result<WebResponse<Response>, Exception<Error>> {
    let res = RepositoryHandler::new(cmd, MockDb).handle().await?;

    Ok(WebResponse(res))
}

#[axum::debug_handler]
pub async fn save_device_temperature(
    Json(cmd): Json<SaveDeviceTemperatureBody>,
) -> Result<WebResponse<Response>, Exception<Error>> {
    let res = RepositoryHandler::new(cmd.into_command()?, MockDb)
        .handle()
        .await?;

    Ok(WebResponse(res))
}

pub fn routers() -> Router {
    Router::new()
        .route("/device_groups", post(register_device_group))
        .route(
            "/devices",
            post(register_device).patch(save_device_temperature),
        )
}
