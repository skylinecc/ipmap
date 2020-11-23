#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_include_static_resources;
extern crate etherparse;
extern crate pcap;
#[macro_use]
extern crate rocket;

use casual_logger::{Level, Log, Opt};
use once_cell::sync::Lazy;
use clap::{App, Arg};
use std::{process::exit, thread, sync::RwLock};
use users::{get_user_by_uid, get_current_uid};

mod ip;
mod web;

pub static IP_MAP: Lazy<RwLock<Vec<[String; 3]>>> = once_cell::sync::Lazy::new(|| {
    RwLock::new(vec!([String::new(), String::new(), String::new()]))
});

fn main() {
    let user = get_user_by_uid(get_current_uid()).unwrap();

    if user.name().to_string_lossy() != "root" {
        eprintln!("ipmap: you must be root to execute ipmap.");
        exit(5);
    }

    // Set application details
    let app = App::new("ipmap")
        .version("0.1.0")
        .author("Skyline High School Coding Club Authors <skylinecc@gmail.com>")
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .help("Launches the program without opening the browser")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("service")
                .long("service")
                .short("s")
                .help("Choose what service to get IP locations from. Must either be 'ipwhois' or 'ip-api'.")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    // Set log settings
    Log::set_opt(Opt::Release);
    Log::remove_old_logs();
    Log::set_level(Level::Notice);

    if app.is_present("service") && app.value_of("service") == Some("ipwhois") {
        thread::spawn(|| {
            ip::ipextract("ipwhois");
        });
    } else if app.is_present("service") && app.value_of("service") == Some("ip-api") {
        thread::spawn(|| {
            ip::ipextract("ip-api");
        });
    } else if !app.is_present("service") {
        thread::spawn(|| {
            ip::ipextract("ipwhois");
        });
    } else if app.is_present("service") {
        let error = format!("\"service\" must be either ipwhois or ip-api");
        eprintln!("{}", error);
        Log::error(&format!("{}", error));
        exit(1);
    };

    // Run page.html in another thread IF the headless option is not used.
    if !app.is_present("headless") {
            web::rocket();
    };
}
