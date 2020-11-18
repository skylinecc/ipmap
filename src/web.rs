use rocket::response::content;

pub fn rocket() {
    println!("Running Webserver");
    rocket::ignite().mount("/", routes![index, json]).launch();
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(format!("{}", include_str!("index.html")))
}

#[get("/json")]
fn json() -> &'static str {
    "haahahaha please help me JSON is so hard..."
}
