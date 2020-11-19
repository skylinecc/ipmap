use rocket::response::content;
use rocket_include_static_resources::{StaticResponse, static_response, static_resources_initialize};

pub fn rocket() {
    println!("Running Webserver");
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "icon",
                "data/icon.png",
            );
        }))
        .mount("/", routes![index, icon, json])
        .launch();
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(format!("{}", include_str!("../data/index.html")))
}

#[get("/json")]
fn json() -> &'static str {
    "haahahaha please help me JSON is so hard..."
}

#[get("/icon.png")]
fn icon() -> StaticResponse {
    static_response!("icon")
}
