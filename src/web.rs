use rocket::{
    config::{Config, Environment, LoggingLevel},
    response::content,
};
use rocket_include_static_resources::{
    static_resources_initialize, static_response, StaticResponse,
};
use serde::{Deserialize, Serialize};

use crate::IP_MAP;

pub fn rocket() {
    println!("Running Webserver");

    let config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(700)
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
        println!("Launched webserver at localhost:700");
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

#[derive(Serialize, Deserialize)]
struct IPAddress {
    ip: String,
    latitude: String,
    longitude: String,
}

#[get("/map.json")]
fn json() -> content::Json<String> {
    let json = get_document();

    content::Json(json)
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

pub fn get_document() -> String {
    let mut json: String = String::new();

    json.push_str("[\n");

    for a in &*IP_MAP.read().unwrap() {
        let address = IPAddress {
            ip: a[0].to_owned(),
            latitude: a[1].to_owned(),
            longitude: a[2].to_owned(),
        };

        let serialized = match serde_json::to_string(&address) {
            Ok(data) => data,
            Err(error) => {
                let error_string = format!("Error serializing JSON: {}", error);
                eprintln!("{}", error_string);
                error_string
            }
        };

        json.push_str(&format!("{},\n", serialized));
    }

    json = (&json[0..json.len() - 2]).to_string();
    json.push_str("\n]\n");
    json
}
