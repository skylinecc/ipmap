extern crate etherparse;
extern crate pcap;

use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use serde_json::Value;
use ureq;

struct Locator {}

impl Locator {
    pub fn get(ip: String) -> std::result::Result<(f64, f64), String> {
        let url = format!("http://ipwhois.app/json/{}", ip);

        let response = ureq::get(&url).call();

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
            Value::Number(latitude) => latitude,
            _ => {
                return Err("Unable to find latitude in parsed JSON".to_string());
            }
        };

        let latitude = match latitude.as_f64() {
            Some(f64_value) => f64_value,
            None => {
                return Err(format!("Unexpected latitude. Not a float 64"));
            }
        };

        let longitude = match &parsed_json["longitude"] {
            Value::Number(longitude) => longitude,
            _ => {
                return Err("Unable to find longitude in parsed JSON".to_string());
            }
        };

        let longitude = match longitude.as_f64() {
            Some(f64_value) => f64_value,
            None => {
                return Err(format!("Unexpected latitude. Not a float 64"));
            }
        };

        let result = (longitude, latitude);

        Ok(result)
    }
}

fn main() {
    let mut cap = Device::lookup().unwrap().open().unwrap();

    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(value) => println!("IP error {:?}", value),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                    println!("IP Address: {:?}", header.source_addr());
                    match Locator::get(format!("{}", header.source_addr())) {
                    	Ok(data) => {
                    		println!("Latitude: {}", data.0);
							println!("Longitude: {}", data.1);
							data
                    	}
                    	Err(error) => {
                    		eprintln!("ERROR: {}", error);
                    		(0.0, 0.0)
                    	}
                    };
                }
                Some(_) | None => (),
            },
        }
    }
}
