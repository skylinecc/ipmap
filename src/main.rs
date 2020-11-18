#![feature(proc_macro_hygiene, decl_macro)]

extern crate etherparse;
extern crate pcap;
#[macro_use]
extern crate rocket;

use casual_logger::{Level, Log, Opt};
use clap::{App, Arg};
use std::thread;

mod ip;
mod locator;
mod web;

fn main() {
    // Set application details
    let app = App::new("ipmap")
        .version("0.1.0")
        .author("Skyline High School Coding Club Authors <skylinecc@gmail.com")
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .help("Launches the program without opening the browser")
                .required(false)
                .takes_value(false),
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

    ip::ipextract();
}
