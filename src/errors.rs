use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ErrorWrapper {
    pub message: String,
    pub status: i32,
}
