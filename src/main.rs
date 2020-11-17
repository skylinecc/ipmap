extern crate etherparse;
extern crate pcap;

use casual_logger::{Level, Log, Opt};
use clap::{App, Arg};
use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use serde_json::json;
use std::{collections::HashSet, fs, include_bytes, io::prelude::*, path::Path};

mod locator;

const INDEX_HTML: &'static [u8] = include_bytes!("index.html");

fn main() {
    // Set application details
    let app = App::new("ipmap")
        .version("0.1.0")
        .author("Skyline High School Coding Club Authors <skylinecc@gmail.com")
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .help("Launches the program without opening the browser")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    let path = &get_path();

    // Remove temporary files
    if Path::new(&format!("{}ipmap.html", path)).is_file() {
        fs::remove_file(&format!("{}ipmap.html", path)).expect(&format!("Couldn't create {}ipmap.html", path));
    };

    if Path::new(&format!("{}ipmap.json", path)).is_file() {
        fs::remove_file(&format!("{}ipmap.json", path)).expect(&format!("Couldn't create {}ipmap.json", path));
    };

    // Run page.html in another thread IF the headless option is not used.
    if !app.is_present("headless") {
        let mut file =
            std::fs::File::create(&format!("{}ipmap.html", path)).expect(&format!("Couldn't create {}ipmap.html", path));
        file.write_all(INDEX_HTML)
            .expect("Couldn't write to ipmap.html");

        open::that_in_background(&format!("{}ipmap.html", path));
    }

    let mut mapdata =
        std::fs::File::create(&format!("{}ipmap.json", path)).expect(&format!("Couldn't create {}ipmap.json", path));
    let mut ip_index = HashSet::new();
    let mut latitude_index = HashSet::new();
    let mut longitude_index = HashSet::new();

    // Set log settings
    Log::set_opt(Opt::Release);
    Log::remove_old_logs();
    Log::set_level(Level::Notice);

    let mut cap = Device::lookup().unwrap().open().unwrap();

    // Loop through each packet in the capture interface as an iterator until it returns an error.
    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(error) => Log::error(&error.to_string()),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                    let current_ip = header.source_addr();
                    if !ip_index.contains(&current_ip.to_string()) && !current_ip.is_private() {
                        ip_index.insert(current_ip.to_string());
                        // Run locator with the IP address, which returns Latitude and Longitude.
                        match locator::Locator::get(current_ip.to_string()) {
                            Ok(ip) => {
                                if !latitude_index.contains(&ip.longitude) {
                                    if !longitude_index.contains(&ip.longitude) {
                                        let json = json!({
                                            "location": {
                                                "ip": ip.ip,
                                                "latitude": ip.latitude,
                                                "longitude": ip.longitude,
                                            }
                                        });
                                        longitude_index.insert(ip.longitude);
                                        println!("{} - {}", ip.ip, ip.city);
                                        mapdata
                                            .write_all(format!("\n{}", json).as_bytes())
                                            .expect("Couldn't write to /tmp/ipmap.json");
                                    }
                                    latitude_index.insert(ip.latitude);
                                }
                            }
                            // If there was an error, send it to the logs.
                            Err(error) => {
                                Log::error(&current_ip.to_string());
                                Log::error(&error);
                            }
                        }
                    }
                }
                Some(_) | None => (),
            },
        }
    }
}

// Set path for temporary file based on the operating system
fn get_path() -> String {
    if std::env::consts::OS == "windows" {
        return "%userprofile%\\AppData\\Local\\Temp\\".to_string();
    } else {
        return "/tmp/".to_string();
    }
}
