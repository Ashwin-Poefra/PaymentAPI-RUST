use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct User {
    pub email: String,
    pub name: String,
}
