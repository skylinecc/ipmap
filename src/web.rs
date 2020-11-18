use rocket::response::content;
use rocket::config::{Config, Environment};
use clap::{App, Arg, ArgMatches};

pub fn rocket(app: ArgMatches) {
    println!("Running Webserver");

    if app.is_present("PORT") {
        let config = match Config::build(Environment::Production).port(app.value_of("PORT")).address("127.0.0.1").finalize() {
            Ok(config) => config,
            Err(error) => {
                eprintln!("Error in rocket config: {}", error);
                Config::new(Environment::Production)
            }
        };
        rocket::custom(config)
            .mount("/", routes![index, json])
            .launch();

    } else {
        rocket::ignite()
            .mount("/", routes![index, json])
            .launch();
    };
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(format!("{}", include_str!("index.html")))
}

#[get("/json")]
fn json() -> &'static str {
    "test"
}
