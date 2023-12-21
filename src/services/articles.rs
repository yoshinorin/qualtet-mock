use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/articles")]
async fn articles() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/articles.json")?;
    utils::make_ok_response(content)
}
