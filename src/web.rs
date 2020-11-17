pub fn rocket() {
    println!("Running Webserver");
    rocket::ignite().mount("/", routes![index, json]).launch();
}

#[get("/")]
fn index() -> &'static str {
    include_str!("index.html")
}

#[get("/json")]
fn json() -> &'static str {
    "test"
}
