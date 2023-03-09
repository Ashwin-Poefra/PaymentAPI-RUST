use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use axum::{extract, Json};
use serde::Deserialize;
use validator::Validate;

use crate::config;
use crate::models::payment::Payment;
use crate::pkg::db;
use crate::{errors::Error, models::common::ResponseWrapper};

// pub fn endpoints(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("")
//             .route(web::get().to(get_payments))
//             .route(web::post().to(post_payment)),
//     );
// }

pub async fn post_payment(
    extract::Json(payment): extract::Json<Payment>,
) -> Result<Json<ResponseWrapper>, Error> {
    match payment.validate() {
        Ok(_) => (),
        Err(_) => return Err(Error::bad_request()),
    };

    let client = db::CLIENT.get().await;

    let pk: AttributeValue = AttributeValue::S(format!("uid#{}", payment.uid));
    let sk: AttributeValue = AttributeValue::S(format!("cardno#{}", payment.card.number));

    let mut item: HashMap<String, AttributeValue> = serde_dynamo::to_item(payment).unwrap();

    item.insert(String::from("pk"), pk);
    item.insert(String::from("sk"), sk);

    match client
        .put_item()
        .table_name(config::db::PAYMENTS_TABLE_NAME.clone())
        .set_item(Some(item))
        .send()
        .await {
            Ok(_) => (),
            Err(_) => return Err(Error::bad_request()),
        };

    return Ok(Json(ResponseWrapper {
        message: String::from("success"),
    }));
}

#[derive(Deserialize)]
pub struct Params {
    uid: String,
}

pub async fn get_payments(params: extract::Query<Params>) -> Result<Json<Vec<Payment>>, Error> {
    let client = db::CLIENT.get().await;

    // Get documents from DynamoDB
    let result = client
        .query()
        .table_name(config::db::PAYMENTS_TABLE_NAME.clone())
        .key_condition_expression("#pk = :pk")
        .expression_attribute_names("#pk", "pk")
        .expression_attribute_values(":pk", AttributeValue::S(format!("uid#{}", params.uid)))
        .send()
        .await
        .unwrap();

    // And deserialize them as strongly-typed data structures
    if let Some(items) = result.items().map(|slice| slice.to_vec()) {
        let users: Vec<Payment> = serde_dynamo::from_items(items).unwrap();
        return Ok(Json(users));
    }

    return Ok(Json(vec![]));
}
