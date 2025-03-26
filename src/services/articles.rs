use crate::utils;
use actix_web::{Error, HttpResponse, get};

#[get("/v1/articles/")]
async fn articles() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/articles/index.json")?;
    utils::make_ok_response(content)
}
