use locator::Locator;
use casual_logger::Log;
use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use std::{collections::HashSet, sync::Arc, sync::Mutex};

pub fn ipextract() {
    println!("Running IP Detection");

    let mut ip_index = HashSet::new();
    let mut latitude_index = HashSet::new();
    let mut longitude_index = HashSet::new();

    let mut cap = Device::lookup().unwrap().open().unwrap();

    // Loop through each packet in the capture interface as an iterator until it returns an error.
    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(error) => {
                Log::error(&error.to_string());
            }
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                    let current_ip = header.source_addr();
                    if !ip_index.contains(&current_ip.to_string()) && !current_ip.is_private() {
                        ip_index.insert(current_ip.to_string());

                        // Run locator with the IP address, which returns Latitude and Longitude.
                        match Locator::get(current_ip.to_string()) {
                            Ok(ip) => {
                                if !latitude_index.contains(&ip.longitude) {
                                    if !longitude_index.contains(&ip.longitude) {
                                        let json_index: Arc<
                                            Mutex<HashSet<(String, String, String)>>,
                                        > = Arc::new(Mutex::new(HashSet::new()));

                                        json_index.lock().unwrap().insert((
                                            ip.ip.clone(),
                                            ip.latitude.clone(),
                                            ip.longitude.clone(),
                                        ));

                                        println!("{} ({})", ip.ip, ip.city);
                                        longitude_index.insert(ip.longitude);
                                    }
                                    latitude_index.insert(ip.latitude);
                                }
                            }
                            // If there was an error, send it to the logs.
                            Err(error) => {
                                eprintln!("locator() error: {} ({})", current_ip.to_string(), error);
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
