use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use serde_json::json;
use std::collections::HashSet;
use casual_logger::Log;
use crate::locator::Locator;

pub fn ipextract() {
    println!("Running IP Detection");

    let mut ip_index = HashSet::new();
    let mut latitude_index = HashSet::new();
    let mut longitude_index = HashSet::new();

//    println!("running device lookup");
    let mut cap = Device::lookup().unwrap().open().unwrap();
//    println!("finish device lookup");

    // Loop through each packet in the capture interface as an iterator until it returns an error.
    while let Ok(packet) = cap.next() {
//        println!("got a packet");
        match SlicedPacket::from_ethernet(packet.data) {
            Err(error) => {
                Log::error(&error.to_string());
//                println!("error getting data from SlicedPacket");
            }
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                    let current_ip = header.source_addr();
//                    println!("got an IP... {}", current_ip.to_string());
                    if !ip_index.contains(&current_ip.to_string()) && !current_ip.is_private() {
//                        println!("Got new IP {}, running the locator", current_ip.to_string());
                        ip_index.insert(current_ip.to_string());

                        // Run locator with the IP address, which returns Latitude and Longitude.
                        match Locator::get(current_ip.to_string()) {
                            Ok(ip) => {
//                                println!("ran the locator and it worked");
                                if !latitude_index.contains(&ip.longitude) {
                                    if !longitude_index.contains(&ip.longitude) {
//                                        println!("unique IP location was found");
                                        let json = json!({
                                            "location": {
                                                "ip": ip.ip,
                                                "latitude": ip.latitude,
                                                "longitude": ip.longitude,
                                            }
                                        });
                                        println!("{}", json);
                                        longitude_index.insert(ip.longitude);
                                    }
                                    latitude_index.insert(ip.latitude);
                                }
                            }
                            // If there was an error, send it to the logs.
                            Err(error) => {
                                println!("ERROR: {} ({})", current_ip.to_string(), error);
                                Log::error(&current_ip.to_string());
                                Log::error(&error);
                            }
                        }
                    } else {
                        continue;
                    }
                }
                Some(_) | None => (),
            },
        }
    }
}
