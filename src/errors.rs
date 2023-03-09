use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    IntervalServerError(#[from] IntervalServerError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 400),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 404),

            Error::IntervalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
        }
    }

    pub fn bad_request() -> Self {
        Error::BadRequest(BadRequest {})
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}

#[derive(thiserror::Error, Debug)]
#[error("Interval Server Error")]
pub struct IntervalServerError {}
