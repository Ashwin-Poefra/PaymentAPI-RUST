use std::collections::HashMap;

use crate::config;
use crate::models::common::EncryptedPayload;
use crate::models::payment::{MaskedCard, Payment};
use crate::pkg::{db, keyvault};
use crate::utils::authorization::authorize;
use crate::utils::payment::{categorize_card_number, get_last_4_digits};
use crate::{errors::Error, models::common::ResponseWrapper};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_kms::model::EncryptionAlgorithmSpec;
use aws_sdk_kms::types::Blob;
use axum::http::HeaderMap;
use axum::{extract, Json};
use uuid::Uuid;
use validator::Validate;

pub async fn method(
    headers: HeaderMap,
    extract::Json(payload): extract::Json<EncryptedPayload>,
) -> Result<Json<ResponseWrapper>, Error> {
    let user = match authorize(headers).await {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    let client = keyvault::CLIENT.get().await;

    let cipher = match base64::decode(payload.encrypted_payload.clone()) {
        Ok(result) => result,
        Err(_) => return Err(Error::bad_request()),
    };

    let result = client
        .decrypt()
        .key_id(config::keyvault::RSA_KEY_ID.clone())
        .encryption_algorithm(EncryptionAlgorithmSpec::RsaesOaepSha256)
        .ciphertext_blob(Blob::new(cipher))
        .send()
        .await;

    match result {
        Ok(result) => {
            let plaintext = result.plaintext().expect("could not get plaintext");

            let decrypted = String::from_utf8(plaintext.as_ref().to_vec()).unwrap();
            let payment: Payment = match serde_json::from_str(&decrypted) {
                Ok(result) => result,
                Err(_) => return Err(Error::bad_request()),
            };

            match payment.validate() {
                Ok(_) => (),
                Err(_) => return Err(Error::bad_request()),
            };

            let card_id = Uuid::new_v4();
            let masked = MaskedCard {
                id: card_id.to_string(),
                name: payment.name,
                expiration: payment.card.expiration,
                last_digits: get_last_4_digits(&payment.card.number),
                network: categorize_card_number(&payment.card.number),
            };

            let pk: AttributeValue = AttributeValue::S(format!("uid#{}", user.email));
            let sk: AttributeValue = AttributeValue::S(format!("card#{}", card_id));

            let mut item: HashMap<String, AttributeValue> = serde_dynamo::to_item(masked).unwrap();
            item.insert(String::from("pk"), pk);
            item.insert(String::from("sk"), sk);

            item.insert(
                String::from("encrypted"),
                AttributeValue::S(payload.encrypted_payload),
            );

            let client = db::CLIENT.get().await;
            match client
                .put_item()
                .table_name(config::db::PAYMENTS_TABLE_NAME.clone())
                .set_item(Some(item))
                .send()
                .await
            {
                Ok(_) => (),
                Err(error) => {
                    log::error!("PutItem Error: {:?}", error);
                    return Err(Error::internal_server_error());
                }
            };

            return Ok(Json(ResponseWrapper {
                message: String::from("success"),
            }));
        }

        Err(error) => {
            log::error!("Decrypt Error: {:?}", error);
            return Err(Error::internal_server_error());
        }
    }
}
