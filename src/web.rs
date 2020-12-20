use crate::ip::get_document;

use actix_web::{
    dev::BodyEncoding, get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

static MARKER_SHADOW: &[u8] = include_bytes!("../data/marker-shadow.png");
static MARKER_ICON_2X: &[u8] = include_bytes!("../data/marker-icon-2x.png");
static MARKER_ICON: &[u8] = include_bytes!("../data/marker-icon.png");
static ICON: &[u8] = include_bytes!("../data/icon.png");

#[actix_web::main]
pub async fn webserv(port: u16) -> std::io::Result<()> {
    println!("Starting application at localhost:{}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
            .service(json)
            .service(js)
            .service(leafletcss)
            .service(leafletjs)
            .service(license)
            .service(jquery)
            .service(marker_shadow)
            .service(marker_icon2x)
            .service(marker_icon)
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

#[get("/leaflet.css")]
fn leafletcss() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(include_str!("../data/leaflet.css"))
}

#[get("/leaflet.js")]
fn leafletjs() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(include_str!("../data/leaflet.js"))
}

#[get("/license")]
fn license() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(include_str!("../data/license.html"))
}

#[get("/jquery.min.js")]
fn jquery() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(include_str!("../data/jquery.min.js"))
}

#[get("/icon.png")]
fn icon() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .header("Content-Type", "image/png")
        .body(ICON)
}

#[get("/images/marker-icon.png")]
fn marker_icon() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .header("Content-Type", "image/png")
        .body(MARKER_ICON)
}

#[get("/images/marker-icon-2x.png")]
fn marker_icon2x() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .header("Content-Type", "image/png")
        .body(MARKER_ICON_2X)
}

#[get("/images/marker-shadow.png")]
fn marker_shadow() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .header("Content-Type", "image/png")
        .body(MARKER_SHADOW)
}

#[get("/map.json")]
fn json() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body(get_document())
}
