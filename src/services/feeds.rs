use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/feeds/index")]
pub async fn index_feed() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/feeds/index.json")?;
    utils::make_ok_response(content)
}
