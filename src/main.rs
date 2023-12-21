use actix_web::{App, HttpServer};
use services::{archives::archives, articles::articles, index::index};
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(archives)
            .service(articles)
    })
    .bind(("127.0.0.1", 9002))?
    .shutdown_timeout(3)
    .run()
    .await
}
