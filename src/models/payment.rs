use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct Payment {
    pub name: String,
    #[validate]
    pub card: Card,
    #[validate]
    pub address: Address,
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct MaskedCard {
    pub id: String,
    pub name: String,
    #[validate(length(equal = 4))]
    pub last_digits: String,
    #[validate(length(equal = 7))]
    pub expiration: String,
    pub network: String,
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct Card {
    #[validate(credit_card)]
    pub number: String,
    #[validate(length(min = 3, max = 4))]
    pub cvv: Option<String>,
    #[validate(length(equal = 7))]
    pub expiration: String,
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct Address {
    pub line_1: String,
    pub line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
}
