use std::env;
use dotenv::dotenv;
use api::user_tweets;
use actix_web::{middleware, web, App, HttpServer};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Load environment vars
    let host = env::var("HOST")
        .expect("HOST must be set");

    let port = env::var("PORT")
        .expect("PORT must be set");


    // Log info
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::scope("/api/v1").service(user_tweets))
    }).bind(format!("{}:{}", host, port))?.run().await
}