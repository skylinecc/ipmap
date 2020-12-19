#![feature(proc_macro_hygiene, decl_macro)]

extern crate etherparse;
extern crate pcap;
extern crate rocket_include_static_resources;
#[macro_use]
extern crate rocket;

use clap::{crate_version, App, Arg, ArgMatches};
use once_cell::sync::Lazy;
use std::{sync::RwLock, thread};
#[cfg(unix)]
use users::{get_current_uid, get_user_by_uid};

mod ip;
mod web;

pub static WRITE_PATH: Lazy<RwLock<String>> =
    once_cell::sync::Lazy::new(|| RwLock::new(String::new()));

pub static IP_MAP: Lazy<RwLock<Vec<[String; 3]>>> =
    once_cell::sync::Lazy::new(|| RwLock::new(vec![[String::new(), String::new(), String::new()]]));

fn main() {
    #[cfg(unix)]
    {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        if user.name().to_string_lossy() != "root" {
            eprintln!("ipmap: you must be root to execute ipmap.");
            std::process::exit(5);
        }
    }

    // Set application details
    let app = App::new("ipmap")
        .version(crate_version!())
        .author("Skyline High Coding Club Authors")
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .help("Launches the program without running the webserver")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("service")
                .long("service")
                .short("s")
                .help("Choose Geolocation API")
                .required(false)
                .takes_value(true)
                .value_name("SERVICE")
                .possible_value("ipwhois")
                .possible_value("ipapi")
                .possible_value("ipapico")
                .possible_value("freegeoip"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .help("Set webserver port, if not set it defaults to port 700")
                .required(false)
                .takes_value(true)
                .value_name("PORT"),
        )
        .arg(
            Arg::with_name("write-to-file")
                .long("write-to-file")
                .short("w")
                .help("Set path to write JSON to")
                .required(false)
                .takes_value(true)
                .value_name("PATH"),
        )
        .get_matches();

    match app.value_of("write-to-file") {
        Some(path) => {
            WRITE_PATH.write().unwrap().clear();
            WRITE_PATH.write().unwrap().push_str(&path.to_string());

            println!("Writing JSON output to {}", path);
            path.to_string()
        },
        None => String::new(),
    };

    let port = port(app.clone());

    // Run page.html in another thread IF the headless option is not used.
    if !app.is_present("headless") {
        thread::spawn(move || {
            web::rocket(port);
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
