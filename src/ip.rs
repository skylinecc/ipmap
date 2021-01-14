use clap::ArgMatches;
use etherparse::{InternetSlice, SlicedPacket};
use ipgeolocate::Locator;
use pcap::Device;
use std::collections::HashSet;
use crate::IPAddress;

use crate::{IP_INDEX, WRITE_PATH};

pub fn ipextract(app: ArgMatches) {
    println!("Running IP Detection");

    let mut ip_index = HashSet::new();
    let mut latitude_index = HashSet::new();
    let mut longitude_index = HashSet::new();

    #[cfg(unix)]
    let cap = Device::lookup().unwrap();
    #[cfg(windows)]
    let cap = user_select_device();

    let mut cap = cap.open().unwrap();

    // Loop through each packet in the capture interface as an iterator until it returns an error.
    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(error) => {
                eprintln!("{}", &error.to_string());
            }
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                    let current_ip = header.source_addr();
                    if !ip_index.contains(&current_ip.to_string()) && !current_ip.is_private() {
                        ip_index.insert(current_ip.to_string());

                        match app.value_of("service") {
                            Some(service) => {
                                // Run locator with the IP address, which returns Latitude and Longitude.
                                match Locator::get(current_ip.to_string().as_str(), service) {
                                    Ok(ipgeo) => {
                                        if !latitude_index.contains(&ipgeo.longitude.to_string()) {
                                            if !longitude_index.contains(&ipgeo.longitude.to_string()) {
                                                let ip = ipgeo.ip.clone();
                                                let latitude = ipgeo.latitude.clone();
                                                let longitude = ipgeo.longitude.clone();
                                                let city = ipgeo.city.clone();

                                                IP_INDEX.write().unwrap().push(IPAddress {
                                                    ip,
                                                    latitude,
                                                    longitude,
                                                    city,
                                                });

                                                if app.is_present("write-to-file") {
                                                    write_ip();
                                                }

                                                println!("{} ({})", ipgeo.ip, ipgeo.city);
                                                longitude_index.insert(ipgeo.longitude.to_string());
                                            }
                                            latitude_index.insert(ipgeo.latitude.to_string());
                                        }
                                    }
                                    // If there was an error, send it to the logs.
                                    Err(error) => {
                                        eprintln!(
                                            "{} error: {} ({})",
                                            service,
                                            current_ip.to_string(),
                                            error
                                        );
                                    }
                                };
                            },
                            None => {
                                // Run locator with the IP address, which returns Latitude and Longitude.
                                match Locator::get(current_ip.to_string().as_str(), "ipapi") {
                                    Ok(ipgeo) => {
                                        if !latitude_index.contains(&ipgeo.longitude.to_string()) {
                                            if !longitude_index.contains(&ipgeo.longitude.to_string()) {
                                                let ip = ipgeo.ip.clone();
                                                let latitude = ipgeo.latitude.clone();
                                                let longitude = ipgeo.longitude.clone();
                                                let city = ipgeo.city.clone();

                                                IP_INDEX.write().unwrap().push(IPAddress {
                                                    ip,
                                                    latitude,
                                                    longitude,
                                                    city,
                                                });

                                                if app.is_present("write-to-file") {
                                                    write_ip();
                                                }

                                                println!("{} ({})", ipgeo.ip, ipgeo.city);
                                                longitude_index.insert(ipgeo.longitude.to_string());
                                            }
                                            latitude_index.insert(ipgeo.latitude.to_string());
                                        }
                                    }
                                    // If there was an error, send it to the logs.
                                    Err(error) => {
                                        eprintln!(
                                            "ipapi error: {} ({})",
                                            current_ip.to_string(),
                                            error
                                        );
                                    }
                                };
                            },
                        }
                    }
                }
                Some(_) | None => (),
            },
        }
    }
}

#[cfg(windows)]
fn user_select_device() -> Device {
    let mut devices = Device::list().unwrap();
    if devices.is_empty() {
        eprintln!("Found no device to listen on, maybe you need to run as an Adminstrator");
        std::process::exit(1);
    }
    println!("Select which device to listen on: (choose the number of the item)");
    for (i, d) in devices.iter().enumerate() {
        println!("{}: {:?}", i, d);
    }
    use std::io;

    let mut input = String::new();
    let n = loop {
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(n) => {
                if n < devices.len() {
                    break n;
                } else {
                    println!("Invalid choice, try again");
                    input.clear();
                }
            }
            Err(_) => {
                println!("Invalid choice, try again");
                input.clear();
            }
        }
    };
    println!("Listening on {:?}", devices[n]);
    devices.remove(n)
}

pub fn create_ip_json(address: &IPAddress) -> String {
    let serialized = match serde_json::to_string(&address) {
        Ok(data) => data,
        Err(error) => {
            let error_string = format!("Error serializing JSON: {}", error);
            eprintln!("{}", error_string);
            error_string
        }
    };

    return serialized;
}

fn write_ip() {
    let path: String = WRITE_PATH.read().unwrap().clone();

    let json: String = get_document();

    fstream::write_text(path, json, true).unwrap();
}

pub fn get_document() -> String {
    let mut json: String = String::new();

    json.push_str("[\n");

    let v = &*IP_INDEX.read().unwrap();

    let iter = v[1..].iter();

    for address in iter {
        let serialized = create_ip_json(address);

        json.push_str(&format!("{},\n", serialized));
    }

    json = (&json[0..json.len() - 2]).to_string();
    json.push_str("\n]\n");
    json
}
