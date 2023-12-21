use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/series")]
pub async fn series() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/series.json")?;
    utils::make_ok_response(content)
}
