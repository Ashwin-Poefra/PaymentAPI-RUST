use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payment {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sk: Option<String>,
    pub uid: String,
    pub card: Card,
    pub address: Address,
}

#[derive(Deserialize, Serialize)]
pub struct Card {
    pub number: String,
    pub cvv: Option<String>,
    pub expiration: String,
}

#[derive(Deserialize, Serialize)]
pub struct Address {
    pub line_1: String,
    pub line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
}
