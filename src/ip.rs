use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use serde_json::json;
use std::{collections::HashSet, io::prelude::*};
use casual_logger::Log;
use crate::locator::Locator;

pub fn ipextract() {
    println!("Running IP Detection");

    let path = &get_path();


    let mut ip_index = HashSet::new();
    let mut latitude_index = HashSet::new();
    let mut longitude_index = HashSet::new();

    println!("running device lookup");
    let mut cap = Device::lookup().unwrap().open().unwrap();
    println!("finish device lookup");

    // Loop through each packet in the capture interface as an iterator until it returns an error.
    while let Ok(packet) = cap.next() {
        println!("got a packet");
        match SlicedPacket::from_ethernet(packet.data) {
            Err(error) => Log::error(&error.to_string()),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                    let current_ip = header.source_addr();
                    println!("got an IP...");
                    if !ip_index.contains(&current_ip.to_string()) && !current_ip.is_private() {
                        ip_index.insert(current_ip.to_string());
                        // Run locator with the IP address, which returns Latitude and Longitude.

                        println!("running the locator");
                        match Locator::get(current_ip.to_string()) {
                            Ok(ip) => {
                                println!("ran the locator and it worked");
                                if !latitude_index.contains(&ip.longitude) {
                                    if !longitude_index.contains(&ip.longitude) {
                                        println!("unique IP location was found");
                                        let json = json!({
                                            "location": {
                                                "ip": ip.ip,
                                                "latitude": ip.latitude,
                                                "longitude": ip.longitude,
                                            }
                                        });
                                        println!("{} - {}", ip.ip, ip.city);
                                        longitude_index.insert(ip.longitude);
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
pub fn get_path() -> String {
    println!("runing get_path()");
    if std::env::consts::OS == "windows" {
        return "%userprofile%\\AppData\\Local\\Temp\\".to_string();
    } else {
        return "/tmp/".to_string();
    }
}
