use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/contents/standard")]
pub async fn standard() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/standard.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/empty-robots")]
pub async fn empty_robots() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/emptyRobots.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/empty-tags")]
pub async fn empty_tags() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/emptyTags.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/partially-robots")]
pub async fn partially_robots() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/partiallyRobots.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/with-externalresources")]
pub async fn with_externalresources() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/withExternalResources.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/without-robots")]
pub async fn without_robots() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/withoutRobots.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/without-tags")]
pub async fn without_tags() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/withoutTags.json")?;
    utils::make_ok_response(content)
}
