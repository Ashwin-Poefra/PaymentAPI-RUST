use aws_sdk_dynamodb::model::AttributeValue;
use axum::{extract, Json};
use serde::Deserialize;

use crate::config;
use crate::errors::Error;
use crate::models::payment::MaskedCard;
use crate::pkg::db;

#[derive(Deserialize)]
pub struct Params {
    uid: String,
}

pub async fn method(params: extract::Query<Params>) -> Result<Json<Vec<MaskedCard>>, Error> {
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
        let users: Vec<MaskedCard> = serde_dynamo::from_items(items).unwrap();
        return Ok(Json(users));
    }

    return Ok(Json(vec![]));
}
