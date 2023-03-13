use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct PublicKey {
    pub public_key: String,
}
