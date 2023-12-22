use actix_web::{App, HttpServer};
use services::{
    archives::archives,
    articles::articles,
    index::index,
    series::series,
    system::{health, metadata},
    tags::{tag_a, tags},
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
            .service(tags)
            .service(tag_a)
    })
    .bind(("127.0.0.1", 9002))?
    .shutdown_timeout(3)
    .run()
    .await
}
