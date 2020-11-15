use serde_json::Value;
use ureq::get;

pub struct Locator {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
}

impl Locator {
    pub fn get(ip: String) -> std::result::Result<Self, String> {
        let url = format!("http://ipwhois.app/json/{}", ip);

        let response = get(&url).call();

        if !response.ok() {
            return Err(format!("Couldn't connect to ipwhois.app"));
        };

        // Turn the data into a string.
        let data = match response.into_string() {
            Ok(data) => data,
            Err(error) => {
                return Err(format!("Error transforming to string: {}", error));
            }
        };

        // Turn the data into parsed_json
        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(format!("Error parsing json: {}", error));
            }
        };

        // Get latitude from parsed_json
        let latitude_str = match &parsed_json["latitude"] {
            Value::String(latitude_str) => latitude_str,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string());
            }
        };

        // Get longitude from parsed_json
        let longitude_str = match &parsed_json["longitude"] {
            Value::String(longitude_str) => longitude_str,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let city_str = match &parsed_json["city"] {
            Value::String(city_str) => city_str,
            _ => {
                return Err("Unable to find city in parsed JSON".to_string());
            }
        };

        let latitude = latitude_str.to_string();
        let longitude = longitude_str.to_string();
        let city = city_str.to_string();

        let result = Locator {
            ip,
            latitude,
            longitude,
            city,
        };

        Ok(result)
    }
}
