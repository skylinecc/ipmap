use iron::{Iron, IronResult, Listening, status};
use iron::error::HttpResult;

const INDEX_HTML: &'static [u8] = include_bytes!("index.html");

struct DataHandler {
    json: RwLock<System>,
}

pub fn start_web_server(port: &str) -> HttpResult<Listening> {
    let mut iron = Iron::new()


}
