use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/archives")]
pub async fn archives() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/archives.json")?;
    utils::make_ok_response(content)
}
