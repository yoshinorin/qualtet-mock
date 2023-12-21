use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/system/metadata")]
pub async fn metadata() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/system/metadata.json")?;
    utils::make_ok_response(content)
}
