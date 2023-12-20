use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
mod utils;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello Qualtet!!")
}

#[get("/archives")]
async fn archives() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/archives.json")?;
    utils::make_ok_response(content)
}

#[get("/articles")]
async fn articles() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/articles.json")?;
    utils::make_ok_response(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root).service(archives).service(articles))
        .bind(("127.0.0.1", 9002))?
        .shutdown_timeout(3)
        .run()
        .await
}
