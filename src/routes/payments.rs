use std::collections::HashMap;

use actix_web::{web, HttpResponse};

use aws_sdk_dynamodb::model::AttributeValue;
use serde::Deserialize;
use validator::Validate;

use crate::models::common::ResponseWrapper;
use crate::models::payment::Payment;
use crate::pkg::db;

pub fn endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(get_payments))
            .route(web::post().to(post_payment)),
    );
}

async fn post_payment(
    payment: web::Json<Payment>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    match payment.validate() {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    };

    let client = db::CLIENT.get().await;

    let pk: AttributeValue = AttributeValue::S(format!("uid#{}", payment.uid));
    let sk: AttributeValue = AttributeValue::S(format!("cardno#{}", payment.card.number));

    let mut item: HashMap<String, AttributeValue> = serde_dynamo::to_item(payment)?;

    item.insert(String::from("pk"), pk);
    item.insert(String::from("sk"), sk);

    client
        .put_item()
        .table_name("pay")
        .set_item(Some(item))
        .send()
        .await?;

    return Ok(HttpResponse::Created().json(web::Json(ResponseWrapper {
        message: String::from("success"),
    })));
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
