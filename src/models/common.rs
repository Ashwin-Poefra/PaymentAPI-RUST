use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize)]
pub struct ResponseWrapper {
    pub message: String,
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct EncryptedPayload {
    pub encrypted_payload: String,
}