use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::prelude::*;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello Qualtet!!")
}

#[get("/archives")]
async fn archives() -> Result<HttpResponse, Error> {
    let mut file: File = File::open("./src/resources/archives.json")?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(contents))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root).service(archives))
        .bind(("127.0.0.1", 9002))?
        .shutdown_timeout(3)
        .run()
        .await
}
