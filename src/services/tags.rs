use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/v1/tags/")]
pub async fn tags() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/tags/index.json")?;
    utils::make_ok_response(content)
}

#[get("/v1/tags/tagA")]
pub async fn tag_a() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/tags/tagA.json")?;
    utils::make_ok_response(content)
}
