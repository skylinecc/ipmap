use crate::IPAddress;
use clap::ArgMatches;
use etherparse::{InternetSlice, SlicedPacket};
use ipgeolocate::Locator;
use pcap::Device;

use crate::{IP_INDEX, WRITE_PATH};

pub fn ipextract(app: ArgMatches) {
    println!("Running IP Detection");

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
                    let service = match app.value_of("service") {
                        Some(service) => service,
                        None => {
                            "ipapi"
                        },
                    };
                    
                    let v = (&*IP_INDEX.read().unwrap()).iter().any(|ip| ip.ips.contains(&current_ip.to_string()));

                    if !current_ip.is_private() && !v{
                        handle_ip(service, &current_ip.to_string(), app.is_present("write-to-file"), app.is_present("verbose"));
                    } else {
                        if app.is_present("verbose") {
                            println!("Found redundant or private ip, {}", current_ip.to_string());
                        }
                    }
                    
                }
                Some(_) | None => (),
            },
        }
    }
}

fn handle_ip(service: &str, current_ip: &str, write: bool, verbose: bool) {
    
    match Locator::get(current_ip, service) {
        Ok(ipgeo) => {
            // let curip = IPAddress {
            //     ip: ipgeo.ip.clone(),
            //     latitude: ipgeo.latitude.clone(),
            //     longitude: ipgeo.longitude.clone(),
            //     city: ipgeo.city.clone(),
            // };
            // {
            let v = &mut *IP_INDEX.write().unwrap();
            //     if !v.contains(&curip) {
            //         if verbose {
            //             println!("Adding {} - {} ({}, {}, {})", service.clone(), ipgeo.ip.clone(), ipgeo.city.clone(), ipgeo.latitude.clone(), ipgeo.longitude.clone());
            //         } else {
            //             println!("{} - ({})", ipgeo.ip.clone(), ipgeo.city.clone());
            //         }
            //         v.push(curip);
            //     }
            // }
            let m = v.iter_mut().find(|elem| elem.latitude == ipgeo.latitude && elem.longitude == ipgeo.longitude);
                match m {
                    Some (matchip) => {
                        if verbose {
                            println!("Found duplicate location for {}, adding to internal list", ipgeo.ip);
                        }
                        matchip.ips.push(ipgeo.ip);
                    }
                    None => {
                        println!("Adding {} ({}, {}, {})", ipgeo.ip, ipgeo.city.clone(), ipgeo.latitude.clone(), ipgeo.longitude.clone());
                        v.push(IPAddress {
                            ips: vec![ipgeo.ip],
                            city: ipgeo.city,
                            latitude: ipgeo.latitude,
                            longitude: ipgeo.longitude,
                        })
                    }
                }
            if write {
                if verbose {
                    println!("Writing to JSON to file");
                };
                write_ip();
            };
        }
        Err(error) => {
            eprintln!("{} error: {} ({})", service, current_ip.to_string(), error);
        }
    }
}

#[cfg(windows)]
fn user_select_device() -> Device {
    let mut devices = Device::list().unwrap();
    if devices.is_empty() {
        eprintln!("Found no device to listen on, maybe you need to run as an Administrator");
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
