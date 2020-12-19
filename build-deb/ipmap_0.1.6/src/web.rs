use crate::ip::get_document;

use rocket::{
    config::{Config, Environment, LoggingLevel},
    response::content,
};
use rocket_include_static_resources::{
    static_resources_initialize, static_response, StaticResponse,
};

pub fn rocket(port: u16) {
    println!("Launching application at localhost:{}", port);

    let config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(port)
        .log_level(LoggingLevel::Off)
        .workers(12)
        .unwrap();

    rocket::custom(config)
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "icon",
                "data/icon.png",
                "markericon",
                "data/marker-icon.png",
                "markericon2",
                "data/marker-icon-2x.png",
                "markershadow",
                "data/marker-shadow.png",
            );
        }))
        .mount(
            "/",
            routes![
                markershadow,
                markericon,
                markericon2,
                index,
                icon,
                json,
                license,
                js,
                leafletcss,
                leafletjs,
                jquery
            ],
        )
        .launch();
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(format!("{}", include_str!("../data/index.html")))
}

#[get("/map.js")]
fn js() -> content::JavaScript<String> {
    content::JavaScript(format!("{}", include_str!("../data/map.js")))
}

#[get("/leaflet.css")]
fn leafletcss() -> content::Css<String> {
    content::Css(format!("{}", include_str!("../data/leaflet.css")))
}

#[get("/leaflet.js")]
fn leafletjs() -> content::JavaScript<String> {
    content::JavaScript(format!("{}", include_str!("../data/leaflet.js")))
}

#[get("/license")]
fn license() -> content::Html<String> {
    content::Html(format!("{}", include_str!("../data/license.html")))
}

#[get("/jquery.min.js")]
fn jquery() -> content::JavaScript<String> {
    content::JavaScript(format!("{}", include_str!("../data/jquery.min.js")))
}

#[get("/icon.png")]
fn icon() -> StaticResponse {
    static_response!("icon")
}

#[get("/images/marker-icon.png")]
fn markericon() -> StaticResponse {
    static_response!("markericon")
}

#[get("/images/marker-icon-2x.png")]
fn markericon2() -> StaticResponse {
    static_response!("markericon2")
}

#[get("/images/marker-shadow.png")]
fn markershadow() -> StaticResponse {
    static_response!("markershadow")
}

#[get("/map.json")]
fn json() -> content::Json<String> {
    let json = get_document();

    content::Json(json)
}
