use serde_json::Value;
use ureq::get;
pub struct Locator {}

impl Locator {
    pub fn get(ip: String) -> std::result::Result<(String, String), String> {
        let url = format!("http://ipwhois.app/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            eprintln!("Cannot connect to ipwhois.app");
        };

        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(format!("Error transforming to string: {}", error));
            }
        };

        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(format!("Error parsing json: {}", error));
            }
        };

        let latitude = match &parsed_json["latitude"] {
            Value::String(latitude) => latitude,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string());
            }
        };

        let longitude = match &parsed_json["longitude"] {
            Value::String(longitude) => longitude,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let result = (longitude.clone(), latitude.clone());

        Ok(result)
    }
}