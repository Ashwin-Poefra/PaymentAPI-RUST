use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};

mod errors;
mod models;
mod pkg;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    lazy_static::initialize(&pkg::db::CLIENT);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/payments").configure(routes::payments::endpoints))
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(errors::ErrorWrapper {
                        status: 400,
                        message: String::from("Bad Request"),
                    }),
                )
                .into()
            }))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
