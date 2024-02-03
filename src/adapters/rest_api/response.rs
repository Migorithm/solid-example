use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

use crate::domain::response::{Error, Response};

#[derive(Serialize)]
pub struct WebResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for WebResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(json!(self)).into_response()
    }
}

impl From<Response> for WebResponse<Response> {
    fn from(value: Response) -> Self {
        WebResponse(value)
    }
}

pub struct Exception<T>(pub T);

impl IntoResponse for Exception<Error> {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self.0 {
            err @ Error::NotFound => (StatusCode::NOT_FOUND, format!("{:?}", err)),
            err @ Error::ConversionFailed => (StatusCode::BAD_REQUEST, format!("{:?}", err)),
            err @ Error::SchemaError => (StatusCode::UNPROCESSABLE_ENTITY, format!("{:?}", err)),
            err @ Error::DuplicateKeyError => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
            }
        };

        let body = Json(json!({"error": error_message}));
        eprintln!("{:?}", body);
        (status, body).into_response()
    }
}

impl From<Error> for Exception<Error> {
    fn from(value: Error) -> Self {
        Exception(value)
    }
}
