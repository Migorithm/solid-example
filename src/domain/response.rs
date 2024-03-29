use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
    ConversionFailed,
    NotFound,
    DuplicateKeyError,
    SchemaError,
}

#[derive(Debug, Serialize)]
pub enum Response {
    Empty,
}

impl From<()> for Response {
    fn from(_value: ()) -> Self {
        Self::Empty
    }
}
