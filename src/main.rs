#![feature(proc_macro_hygiene, decl_macro)]

extern crate etherparse;
extern crate pcap;
extern crate rocket_include_static_resources;
#[macro_use]
extern crate rocket;

use casual_logger::{Level, Log, Opt};
use clap::{App, Arg};
use once_cell::sync::Lazy;
use std::{process::exit, sync::RwLock, thread};
#[cfg(unix)]
use users::{get_current_uid, get_user_by_uid};

mod ip;
mod web;

const VERSION: &'static str = "0.1.2";

pub static IP_MAP: Lazy<RwLock<Vec<[String; 3]>>> =
    once_cell::sync::Lazy::new(|| RwLock::new(vec![[String::new(), String::new(), String::new()]]));

#[cfg(windows)]
struct User {}
#[cfg(windows)]
impl User {
    fn name(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new("windows-uknown")
    }
}

fn main() {
    #[cfg(unix)]
    let user = get_user_by_uid(get_current_uid()).unwrap();
    #[cfg(windows)]
    let user = User {};

    if user.name().to_string_lossy() != "root" {
        eprintln!("ipmap: you must be root to execute ipmap.");
        exit(5);
    }

    // Set application details
    let app = App::new("ipmap")
        .version(VERSION)
        .author("Skyline High School Coding Club Authors <skylinecc@gmail.com>")
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
                .help("Geolocation API")
                .required(false)
                .takes_value(true)
                .value_name("SERVICE"),
        )
        .get_matches();

    // Set log settings
    Log::set_opt(Opt::Release);
    Log::remove_old_logs();
    Log::set_level(Level::Notice);

    // Run page.html in another thread IF the headless option is not used.
    if !app.is_present("headless") {
        thread::spawn(|| {
            web::rocket();
        });
    };

    ip::ipextract(app);
}
