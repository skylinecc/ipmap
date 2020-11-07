extern crate etherparse;
extern crate pcap;

use serde_json::Value;
use std::rc::Rc;
use ureq;
use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;

fn main() {
    let mut cap = Device::lookup().unwrap().open().unwrap();

    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => println!("{:?}", header.source_addr()),
                Some(_) | None => (),
            },
        }
    }
}

struct Locator {
    latitude: f64,
    longitude: f64,
}

impl Locator {
    pub fn get(ip: str) -> Rc<Self> {
        let url = format!(
            "http://ipwhois.app/json/",
            ip
        );

		let response = ureq::get(&url).call();

    	if !response.ok() {
			eprintln!("Cannot connect to ipwhois.app");
        };

		let data = match response.into_string() {
			Ok(data) => data,
			Err(error) => {
				eprintln!("Error transforming to string: {}", error);
			}
		};

		let parsed_json: Value = match serde_json::from_str(&data) {
			Ok(parsed_json) => parsed_json,
			Err(error) => {
				eprintln!("Error parsing json: {}", error);
			}
    	};

		let latitude = match &parsed_json["latitude"] {
			Value::Number(latitude) => latitude,
			_ => {
				eprintln!("Unable to find latitude in parsed JSON");
			}
		};

		let latitude = match latitude.as_f64() {
			Some(f64_value) => f64_value,
			None => {
				eprintln!("Unexpected latitude. Not a float 64");
			}
		};

		let longitude = match &parsed_json["longitude"] {
			Value::Number(longitude) => longitude,
			_ => {
				eprintln!("Unable to find longitude in parsed JSON");
			}
		};

		let longitude = match latitude.as_f64() {
			Some(f64_value) => f64_value,
			None => {
				eprintln!("Unexpected longitude. Not a float 64");
			}
		};

        let result = Rc::new(Locator {
            latitude,
            longitude,
        });

        return result;
    }
}
