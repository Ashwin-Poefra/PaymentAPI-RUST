use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseWrapper {
    pub message: String,
}
