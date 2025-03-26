use crate::utils;
use actix_web::{Error, HttpResponse, get};

#[get("/v1/archives/")]
pub async fn archives() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/archives/index.json")?;
    utils::make_ok_response(content)
}
