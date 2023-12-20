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
        .body(content))
}
