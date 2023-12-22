use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;
use services::{
    archives::archives,
    articles::articles,
    contents::{
        empty_robots, empty_tags, partially_robots, standard, with_externalresources,
        without_robots, without_tags,
    },
    index::index,
    series::series,
    system::{health, metadata},
    tags::{tag_a, tags},
};
use std::env;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("server is starting up");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(archives)
            .service(articles)
            .service(standard)
            .service(empty_robots)
            .service(empty_tags)
            .service(partially_robots)
            .service(with_externalresources)
            .service(without_robots)
            .service(without_tags)
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
