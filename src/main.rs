use std::env;
use dotenv::dotenv;
use api::{user_tweets, user_get_tweet, user_post_tweet};
use actix_web::{middleware, web, App, HttpServer, guard, HttpResponse};

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

    // Server
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(user_tweets)
            .service(user_get_tweet)
            .service(user_post_tweet)
            .default_service(
                web::route().guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::NotFound())
            )
    }).bind(format!("{}:{}", host, port))?.run().await
}