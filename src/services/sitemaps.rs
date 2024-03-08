use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/v1/sitemaps/")]
pub async fn sitemaps() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/sitemaps/index.json")?;
    utils::make_ok_response(content)
}
