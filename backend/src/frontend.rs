/*---------------------------------------------------------------------------------------------
 *  Copyright 2024 SES
 *  Licensed under the Apache 2.0 License. See LICENSE.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use actix_web::{get, web};
use actix_web::{HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "frontend"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => {
            if path.ends_with("index.html") {
                Asset::get("empty.html")
                    .map(|content| {
                        HttpResponse::Ok()
                            .content_type(from_path("empty.html").first_or_octet_stream().as_ref())
                            .body(content.data.into_owned())
                    })
                    .unwrap_or_else(|| HttpResponse::NotFound().body("404 Not Found"))  
            } else {
                HttpResponse::NotFound().body("404 Not Found")
            }
        }
    }
}

#[get("/")]
pub async fn index() ->  impl Responder {
    handle_embedded_file("index.html")
}

#[get("/{_:.*.html|favicon.ico|_nuxt.*}")]
pub async fn embedded_file(path: web::Path<String>) ->  impl Responder {
    handle_embedded_file(path.as_str())
}
