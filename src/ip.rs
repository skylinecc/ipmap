use ipgeolocate::Locator;
use casual_logger::Log;
use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use std::collections::HashSet;
use clap::ArgMatches;

use crate::IP_MAP;

pub fn ipextract(app: ArgMatches) {
    println!("Running IP Detection");

    if app.is_present("output") {

    };

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

                        if app.value_of("service") == Some("ipwhois") {
                            // Run locator with the IP address, which returns Latitude and Longitude.
                            match Locator::ipwhois(current_ip.to_string().as_str()) {
                                Ok(ip) => {
                                    if !latitude_index.contains(&ip.longitude.to_string()) {
                                        if !longitude_index.contains(&ip.longitude.to_string()) {

                                            IP_MAP.write().unwrap().push([ip.ip.clone(), ip.latitude.to_string().clone(), ip.longitude.to_string().clone()]);

                                            println!("{} ({})", ip.ip, ip.city);
                                            longitude_index.insert(ip.longitude.to_string());
                                        }
                                        latitude_index.insert(ip.latitude.to_string());
                                    }
                                }
                                // If there was an error, send it to the logs.
                                Err(error) => {
                                    eprintln!("ipwhois error: {} ({})", current_ip.to_string(), error);
                                    Log::error(&format!("ipwhois error: {} ({})", current_ip.to_string(), error));
                                }
                            }
                        } else if app.value_of("service") == Some("freegeoip") {
                            // Run locator with the IP address, which returns Latitude and Longitude.
                            match Locator::freegeoip(current_ip.to_string().as_str()) {
                                Ok(ip) => {
                                    if !latitude_index.contains(&ip.longitude.to_string()) {
                                        if !longitude_index.contains(&ip.longitude.to_string()) {

                                            IP_MAP.write().unwrap().push([ip.ip.clone(), ip.latitude.to_string().clone(), ip.longitude.to_string().clone()]);

                                            println!("{} ({})", ip.ip, ip.city);
                                            longitude_index.insert(ip.longitude.to_string());
                                        }
                                        latitude_index.insert(ip.latitude.to_string());
                                    }
                                }
                                // If there was an error, send it to the logs.
                                Err(error) => {
                                    eprintln!("freegeoip error: {} ({})", current_ip.to_string(), error);
                                    Log::error(&format!("freegeoip error: {} ({})", current_ip.to_string(), error));
                                }
                            }
                        } else if app.value_of("service") == Some("ipapi") {
                            // Run locator with the IP address, which returns Latitude and Longitude.
                            match Locator::ipapi(current_ip.to_string().as_str()) {
                                Ok(ip) => {
                                    if !latitude_index.contains(&ip.longitude.to_string()) {
                                        if !longitude_index.contains(&ip.longitude.to_string()) {

                                            IP_MAP.write().unwrap().push([ip.ip.clone(), ip.latitude.to_string().clone(), ip.longitude.to_string().clone()]);

                                            println!("{} ({})", ip.ip, ip.city);
                                            longitude_index.insert(ip.longitude.to_string());
                                        }
                                        latitude_index.insert(ip.latitude.to_string());
                                    }
                                }
                                // If there was an error, send it to the logs.
                                Err(error) => {
                                    eprintln!("ipapi error: {} ({})", current_ip.to_string(), error);
                                    Log::error(&format!("ipapi error: {} ({})", current_ip.to_string(), error));
                                }
                            }
                        } else if app.value_of("service") == Some("ipapico") {
                            // Run locator with the IP address, which returns Latitude and Longitude.
                            match Locator::ipapico(current_ip.to_string().as_str()) {
                                Ok(ip) => {
                                    if !latitude_index.contains(&ip.longitude.to_string()) {
                                        if !longitude_index.contains(&ip.longitude.to_string()) {

                                            IP_MAP.write().unwrap().push([ip.ip.clone(), ip.latitude.to_string().clone(), ip.longitude.to_string().clone()]);

                                            println!("{} ({})", ip.ip, ip.city);
                                            longitude_index.insert(ip.longitude.to_string());
                                        }
                                        latitude_index.insert(ip.latitude.to_string());
                                    }
                                }
                                // If there was an error, send it to the logs.
                                Err(error) => {
                                    eprintln!("ipapico error: {} ({})", current_ip.to_string(), error);
                                    Log::error(&format!("ipapico error: {} ({})", current_ip.to_string(), error));
                                }
                            }
                        } else {
                            // Run locator with the IP address, which returns Latitude and Longitude.
                            match Locator::ipapi(current_ip.to_string().as_str()) {
                                Ok(ip) => {
                                    if !latitude_index.contains(&ip.longitude.to_string()) {
                                        if !longitude_index.contains(&ip.longitude.to_string()) {

                                            IP_MAP.write().unwrap().push([ip.ip.clone(), ip.latitude.to_string().clone(), ip.longitude.to_string().clone()]);

                                            println!("{} ({})", ip.ip, ip.city);
                                            longitude_index.insert(ip.longitude.to_string());
                                        }
                                        latitude_index.insert(ip.latitude.to_string());
                                    }
                                }
                                // If there was an error, send it to the logs.
                                Err(error) => {
                                    eprintln!("ipapi error: {} ({})", current_ip.to_string(), error);
                                    Log::error(&format!("ipapi error: {} ({})", current_ip.to_string(), error));
                                }
                            }
                        }
                    }
                }
                Some(_) | None => (),
            },
        }
    }
}
