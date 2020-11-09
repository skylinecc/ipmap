extern crate etherparse;
extern crate pcap;

use casual_logger::{Level, Log, Opt};
use clap::{App, Arg};
use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use serde_json::json;
use std::{collections::HashSet, fs, include_bytes, io::prelude::*, path::Path, thread};

mod locator;

fn main() {
    let app = App::new("IPmap")
        .version("0.1.0")
        .author("Skyline High School Coding Club <skylinecc@gmail.com")
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .help("Launches the program without opening the browser")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    //remove temporary files
    if Path::new("/tmp/ipmap.html").is_file() {
        fs::remove_file("/tmp/ipmap.html").expect("Couldn't remove ipmap.html");
    };

    if Path::new("/tmp/ipmap.json").is_file() {
        fs::remove_file("/tmp/ipmap.json").expect("Couldn't remove sipmap.json");
    };

    // Run page.html in another thread IF the headless option is not used.
    if !app.is_present("headless") {
        thread::spawn(|| {
            let page = include_bytes!("page.html");

            let mut file =
                std::fs::File::create("/tmp/ipmap.html").expect("Couldn't create ipmap.html");
            file.write_all(page)
                .expect("Couldn't write to ipmap.html");

            open::that("/tmp/ipmap.html").expect("Couldn't open ipmap.html");
        });
    }

    let mut mapdata =
        std::fs::File::create("/tmp/ipmap.json").expect("Couldn't create /tmp/ipmap.json");
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
                    let cur_ip = header.source_addr();
                    if !ip_index.contains(&cur_ip.to_string()) && !cur_ip.is_private() {
                        ip_index.insert(cur_ip.to_string());
                        // Run locator with the IP address, which returns Latitude and Longitude.
                        match locator::Locator::get(cur_ip.to_string()) {
                            Ok(data) => {
                                if !latitude_index.contains(&data.longitude) {
                                    if !longitude_index.contains(&data.longitude) {
                                        let json = json!({
                                            "location": {
                                                "ip": cur_ip,
                                                "latitude": data.latitude,
                                                "longitude": data.longitude,
                                            }
                                        });
                                        longitude_index.insert(data.longitude);
                                        println!("{}", json);
                                        mapdata
                                            .write_all(format!("\n{}", json).as_bytes())
                                            .expect("Couldn't write to /tmp/ipmap.json");
                                    }
                                    latitude_index.insert(data.latitude);
                                }
                            }
                            // If there was an error, send it to the logs.
                            Err(error) => {
                                Log::error(&cur_ip.to_string());
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
