use axum::{
    routing::{get, post},
    Router,
};
use lambda_http::Error;

mod config;
mod errors;
mod models;
mod pkg;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .init();

    let app = Router::new()
        .route("/default/wallet", get(routes::payments::get_payments))
        .route("/default/wallet", post(routes::payments::post_payment));

    lambda_http::run(app).await
}
