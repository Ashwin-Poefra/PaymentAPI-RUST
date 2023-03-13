use aws_sdk_kms::{types::Blob, model::EncryptionAlgorithmSpec};
use axum::{Json, extract};

use crate::{config, errors::Error, models::{payment::{Payment, EncryptedPayload}}, pkg::keyvault};

pub async fn post_encrypt(
    extract::Json(payment): extract::Json<Payment>,
) -> Result<Json<EncryptedPayload>, Error> {
    let client = keyvault::CLIENT.get().await;

    let json_string = serde_json::to_string(&payment).unwrap();

    match client
        .encrypt()
        .key_id(config::keyvault::RSA_KEY_ID.clone())
        .encryption_algorithm(EncryptionAlgorithmSpec::RsaesOaepSha256)
        .plaintext(Blob::new(json_string.as_bytes().to_vec()))
        .send()
        .await
    {
        Ok(result) => {
            let blob = result.ciphertext_blob().expect("Could not encrypt");
            let bytes = blob.as_ref();

            return Ok(Json(EncryptedPayload {
                encrypted: base64::encode(bytes),
            }));
        }
        Err(error) => {
            println!("{:?}", error);
            return Err(Error::internal_server_error());
        }
    };
}
