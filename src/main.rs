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
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    env_logger::init();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .init();

    let app = Router::new()
        .route("/default/wallet", get(routes::payments::get_payments::method))
        .route("/default/wallet", post(routes::payments::create_payment::method))
        .route("/default/publickey", get(routes::publickey::get_publickey))
        .route("/default/encrypt", post(routes::encrypt::post_encrypt));

    lambda_http::run(app).await
}
