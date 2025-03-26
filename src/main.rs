use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;
use services::{
    archives::archives,
    articles::articles,
    contents::{
        content_with_trailing_slash, content_without_trailing_slash,
        content_yyyymmdd_with_trailing_slash, content_yyyymmdd_without_trailing_slash,
    },
    feeds::index_feed,
    index::index,
    series::series,
    sitemaps::sitemaps,
    system::{health, metadata},
    tags::{tag_a, tags},
};
use std::env;

mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    info!("server is starting up");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(index)
            .service(archives)
            .service(articles)
            .service(content_with_trailing_slash)
            .service(content_without_trailing_slash)
            .service(content_yyyymmdd_with_trailing_slash)
            .service(content_yyyymmdd_without_trailing_slash)
            .service(index_feed)
            .service(series)
            .service(sitemaps)
            .service(health)
            .service(metadata)
            .service(tags)
            .service(tag_a)
    })
    .bind(("0.0.0.0", 9002))?
    .shutdown_timeout(3)
    .run()
    .await
}
