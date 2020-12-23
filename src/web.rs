extern crate web_view;

use crate::ip::get_document;
use web_view::*;
use std::thread;

use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

static ICON: &[u8] = include_bytes!("../data/icon.png");

#[actix_web::main]
pub async fn webserv(port: u16) -> std::io::Result<()> {
    println!("Starting application at localhost:{}", port);

    thread::spawn(move || {
        web_ui(port.clone());
    });

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
            .service(json)
            .service(js)
            .service(license)
            .service(icon)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .disable_signals()
    .run()
    .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(include_str!("../data/index.html"))
}

#[get("/map.js")]
fn js() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(include_str!("../data/map.js"))
}

#[get("/license")]
fn license() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(include_str!("../data/license.html"))
}

#[get("/icon.png")]
fn icon() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .header("Content-Type", "image/png")
        .body(ICON)
}

#[get("/map.json")]
fn json() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(get_document())
}

fn web_ui(port: u16) {
    web_view::builder()
        .title("Ipmap")
        .content(Content::Url(&format!("http://127.0.0.1:{}", port)))
        .size(800, 600)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
    println!("UI closed, exiting now.");
    std::process::exit(0)
}
