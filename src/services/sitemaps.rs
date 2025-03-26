use crate::utils;
use actix_web::{Error, HttpResponse, get};

#[get("/v1/sitemaps/")]
pub async fn sitemaps() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/sitemaps/index.json")?;
    utils::make_ok_response(content)
}
