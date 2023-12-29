use actix_web::Error as HttpResponseError;
use actix_web::HttpResponse;
use std::fs::File;
use std::io::{self, prelude::*};
use std::result::Result;

pub fn readfile(path: &str) -> Result<String, io::Error> {
    let mut f: File = File::open(path)?;
    let mut contents: String = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn make_ok_response(content: String) -> Result<HttpResponse, HttpResponseError> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("X-Request-Id", "c5befa09-ac43-4ee5-9ece-0db860163d93"))
        .insert_header(("X-Response-Time", "15"))
        .body(content))
}

// TODO: I want to overload with args.
pub fn make_empty_ok_response() -> Result<HttpResponse, HttpResponseError> {
    Ok(HttpResponse::Ok()
        .insert_header(("X-Request-Id", "c5befa09-ac43-4ee5-9ece-0db860163d93"))
        .insert_header(("X-Response-Time", "15"))
        .body(""))
}

pub fn make_not_found_response() -> Result<HttpResponse, HttpResponseError> {
    Ok(HttpResponse::NotFound()
        .insert_header(("X-Request-Id", "c5befa09-ac43-4ee5-9ece-0db860163d93"))
        .insert_header(("X-Response-Time", "15"))
        .body(""))
}
