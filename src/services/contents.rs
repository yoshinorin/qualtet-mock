use crate::utils;
use actix_web::web::Path;
use actix_web::{get, Error, HttpResponse};

#[get("/contents/articles/nested/{path}")]
pub async fn content_without_trailing_slash(path: Path<String>) -> Result<HttpResponse, Error> {
    response(format!("{}{}", path, "/"))
}

#[get("/contents/articles/nested/{path}/")]
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
