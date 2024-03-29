use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/v1/articles/")]
async fn articles() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/articles/index.json")?;
    utils::make_ok_response(content)
}
