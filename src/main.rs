extern crate etherparse;
extern crate pcap;

use clap::{ArgMatches};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::{sync::RwLock, thread};

#[cfg(unix)]
use users::{get_current_uid, get_user_by_uid};

mod ip;
mod web;
pub static WRITE_PATH: Lazy<RwLock<String>> =
    once_cell::sync::Lazy::new(|| RwLock::new(String::new()));

pub static IP_INDEX: Lazy<RwLock<Vec<IPAddress>>> = once_cell::sync::Lazy::new(|| {
    RwLock::new(vec![IPAddress::new()])
});

fn main() {
    #[cfg(unix)]
    {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        if user.name().to_string_lossy() != "root" {
            eprintln!("ipmap: you must be root to execute ipmap.");
            std::process::exit(5);
        }
    }
    
    // Initialize cli app
    let app = ipmap::init_app().get_matches();

    // Set application details
    match app.value_of("write-to-file") {
        Some(path) => {
            WRITE_PATH.write().unwrap().clear();
            WRITE_PATH.write().unwrap().push_str(&path.to_string());

            println!("Writing JSON output to {}", path);
            path.to_string()
        }
        None => String::new(),
    };

    let port = port(app.clone());

    // Run page.html in another thread IF the headless option is not used.
    if !app.is_present("headless") {
        thread::spawn(move || {
            match web::webserv(port) {
                Ok(_) => println!("Finished webserv() successfully"),
                Err(error) => {
                    eprintln!("ERROR starting webserver: {}", error);
                    std::process::exit(1);
                }
            };
        });
    };

    ip::ipextract(app);
}

fn port(app: ArgMatches) -> u16 {
    let port: u16 = match app.value_of("port") {
        Some(port_str) => {
            let num = match port_str.parse::<u16>() {
                Ok(port_data) => port_data,
                Err(error) => {
                    eprintln!("Malformed port argument: {}", error);
                    std::process::exit(1);
                }
            };
            num
        }
        None => 700,
    };
    return port;
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct IPAddress {
    ips: Vec<String>,
    latitude: String,
    longitude: String,
    city: String,
}


impl IPAddress {
    pub fn new() -> Self {
        IPAddress {
            ips: Vec::new(),
            latitude: String::new(),
            longitude: String::new(),
            city: String::new(),
        }
    }
}
