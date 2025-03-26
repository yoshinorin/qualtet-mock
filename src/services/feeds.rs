use crate::utils;
use actix_web::{Error, HttpResponse, get};

#[get("/v1/feeds/index")]
pub async fn index_feed() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/feeds/index.json")?;
    utils::make_ok_response(content)
}
