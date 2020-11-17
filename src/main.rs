#![feature(proc_macro_hygiene, decl_macro)]

extern crate etherparse;
extern crate pcap;
#[macro_use] extern crate rocket;

use casual_logger::{Level, Log, Opt};
use clap::{App, Arg};
use std::{fs, io::prelude::*, path::Path, thread};

mod locator;
mod web;
mod ip;

const INDEX_BYTES: &'static [u8] = include_bytes!("index.html");

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

    let path = &ip::get_path();

    // Remove temporary files
    if Path::new(&format!("{}ipmap.html", path)).is_file() {
        fs::remove_file(&format!("{}ipmap.html", path)).expect(&format!("Couldn't create {}ipmap.html", path));
    };

    if Path::new(&format!("{}ipmap.json", path)).is_file() {
        fs::remove_file(&format!("{}ipmap.json", path)).expect(&format!("Couldn't create {}ipmap.json", path));
    };

    // Run page.html in another thread IF the headless option is not used.
    if !app.is_present("headless") {
        thread::spawn(|| {
            web::rocket();
        });

        let mut file =
            std::fs::File::create(&format!("{}ipmap.html", path)).expect(&format!("Couldn't create {}ipmap.html", path));
        file.write_all(INDEX_BYTES)
            .expect("Couldn't write to ipmap.html");

        open::that_in_background(&format!("{}ipmap.html", path));
    };

    ip::ipextract();
}
