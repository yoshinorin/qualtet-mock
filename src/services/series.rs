use crate::utils;
use actix_web::{Error, HttpResponse, get};

#[get("/v1/series/")]
pub async fn series() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/series/index.json")?;
    utils::make_ok_response(content)
}
