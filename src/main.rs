extern crate diesel
use actix_web::{middleware, web, App, HttpRequest, HttpServer, get};

#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Diesel

    // Load environment vars
    let host = "127.0.0.1";
    let port = "8000";

    // Log info
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::scope("/api/v1").service(index))
    }).bind(format!("{}:{}", host, port))?.run().await
}