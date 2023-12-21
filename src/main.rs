use actix_web::{App, HttpServer};
use services::{
    archives::archives,
    articles::articles,
    index::index,
    series::series,
    system::{health, metadata},
};
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(archives)
            .service(articles)
            .service(series)
            .service(health)
            .service(metadata)
    })
    .bind(("127.0.0.1", 9002))?
    .shutdown_timeout(3)
    .run()
    .await
}
