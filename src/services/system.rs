use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/v1/system/metadata")]
pub async fn metadata() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/system/metadata.json")?;
    utils::make_ok_response(content)
}

#[get("/v1/system/health")]
pub async fn health() -> Result<HttpResponse, Error> {
    utils::make_empty_ok_response()
}
