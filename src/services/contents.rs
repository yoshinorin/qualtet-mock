use crate::utils;
use actix_web::{get, Error, HttpResponse};

#[get("/contents/articles/nested/standard/")]
pub async fn standard() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/nested/standard.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/articles/nested/empty-robots/")]
pub async fn empty_robots() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/nested/emptyRobots.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/articles/nested/empty-tags/")]
pub async fn empty_tags() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/nested/emptyTags.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/articles/nested/partially-robots/")]
pub async fn partially_robots() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/nested/partiallyRobots.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/articles/nested/with-externalresources/")]
pub async fn with_externalresources() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/nested/withExternalResources.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/articles/nested/without-robots/")]
pub async fn without_robots() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/nested/withoutRobots.json")?;
    utils::make_ok_response(content)
}

#[get("/contents/articles/nested/without-tags/")]
pub async fn without_tags() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/nested/withoutTags.json")?;
    utils::make_ok_response(content)
}
