use crate::utils;
use actix_web::web::Path;
use actix_web::{Error, HttpResponse, get};

#[get("/v1/contents/articles/nested/{path}")]
pub async fn content_without_trailing_slash(path: Path<String>) -> Result<HttpResponse, Error> {
    response(format!("{}{}", path, "/"))
}

#[get("/v1/contents/articles/nested/{path}/")]
pub async fn content_with_trailing_slash(path: Path<String>) -> Result<HttpResponse, Error> {
    // NOTE: `actix-web` seems remove trailing slash.
    response(format!("{}{}", path, "/"))
}

fn response(path: String) -> Result<HttpResponse, Error> {
    let json = match path.as_str() {
        "standard/" => utils::readfile("./src/resources/contents/nested/standard.json")?,
        "empty-robots/" => utils::readfile("./src/resources/contents/nested/emptyRobots.json")?,
        "empty-tags/" => utils::readfile("./src/resources/contents/nested/emptyTags.json")?,
        "partially-robots/" => {
            utils::readfile("./src/resources/contents/nested/partiallyRobots.json")?
        }
        "with-externalresources/" => {
            utils::readfile("./src/resources/contents/nested/withExternalResources.json")?
        }
        "without-robots/" => utils::readfile("./src/resources/contents/nested/withoutRobots.json")?,
        "without-tags/" => utils::readfile("./src/resources/contents/nested/withoutTags.json")?,
        _ => return utils::make_not_found_response(),
    };
    utils::make_ok_response(json)
}

#[get("/v1/contents/articles/{yyyy}/{mm}/{dd}/{path}")]
pub async fn content_yyyymmdd_without_trailing_slash() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/yyyy/mm/dd/standard.json")?;
    utils::make_ok_response(content)
}

#[get("/v1/contents/articles/{yyyy}/{mm}/{dd}/{path}/")]
pub async fn content_yyyymmdd_with_trailing_slash() -> Result<HttpResponse, Error> {
    let content = utils::readfile("./src/resources/contents/yyyy/mm/dd/standard.json")?;
    utils::make_ok_response(content)
}
