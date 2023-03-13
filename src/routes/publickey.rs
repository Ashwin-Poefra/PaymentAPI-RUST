use axum::Json;

use crate::{config, errors::Error, models::publickey::PublicKey, pkg::keyvault};

pub async fn get_publickey() -> Result<Json<PublicKey>, Error> {
    let client = keyvault::CLIENT.get().await;

    match client
        .get_public_key()
        .key_id(config::keyvault::RSA_KEY_ID.clone())
        .send()
        .await
    {
        Ok(result) => {
            let blob = result.public_key().expect("Could not get public key");
            let bytes = blob.as_ref();

            return Ok(Json(PublicKey {
                public_key: base64::encode(bytes),
            }));
        }
        Err(error) => {
            println!("{:?}", error);
            return Err(Error::internal_server_error());
        }
    };
}
