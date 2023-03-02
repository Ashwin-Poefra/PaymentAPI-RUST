use std::collections::HashMap;

use actix_web::web;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::Deserialize;

use crate::models::common::MessageResponse;
use crate::models::payment::Payment;
use crate::utils::db;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/payments")
            .route(web::get().to(get_payments))
            .route(web::post().to(post_payment)),
    );
}

async fn post_payment(
    mut payment: web::Json<Payment>,
) -> Result<web::Json<MessageResponse>, Box<dyn std::error::Error>> {
    let client = db::CLIENT.get().await;

    payment.pk = Some(format!("uid#{}", payment.uid));
    payment.sk = Some(format!("cardno#{}", payment.card.number));

    let item: HashMap<String, AttributeValue> = serde_dynamo::to_item(payment)?;

    client
        .put_item()
        .table_name("pay")
        .set_item(Some(item))
        .send()
        .await?;

    return Ok(web::Json(MessageResponse {
        message: format!("success"),
    }));
}

#[derive(Deserialize)]
struct Params {
    uid: String,
}

async fn get_payments(
    params: web::Query<Params>,
) -> Result<web::Json<Vec<Payment>>, Box<dyn std::error::Error>> {
    let client = db::CLIENT.get().await;

    // Get documents from DynamoDB
    let result = client
        .query()
        .table_name("pay")
        .key_condition_expression("#pk = :pk")
        .expression_attribute_names("#pk", "pk")
        .expression_attribute_values(":pk", AttributeValue::S(format!("uid#{}", params.uid)))
        .send()
        .await?;

    // And deserialize them as strongly-typed data structures
    if let Some(items) = result.items().map(|slice| slice.to_vec()) {
        let users: Vec<Payment> = serde_dynamo::from_items(items)?;
        return Ok(web::Json(users));
    }

    return Ok(web::Json(vec![]));
}
