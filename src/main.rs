extern crate etherparse;
extern crate pcap;
extern crate open;

use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use casual_logger::{Level, Log, Opt};
use std::include_bytes;
use std::io::prelude::*;
use std::thread;
use std::fs;
use std::path::Path;

mod locator;

fn main() {
	if Path::new("/tmp/ipmap.html").is_file() {
	 	fs::remove_file("/tmp/ipmap.html").expect("Couldn't remove /tmp/ipmap.html");
	};

	if Path::new("/tmp/ipmap.data").is_file() {
		fs::remove_file("/tmp/ipmap.data").expect("Couldn't remove /tmp/ipmap.data");
	};

	// Run page.html in another thread.
	thread::spawn(|| {
		let page = include_bytes!("page.html");

		let mut file = std::fs::File::create("/tmp/ipmap.html").expect("Couldn't create /tmp/ipmap.html");
		file.write_all(page).expect("Couldn't write to /tmp/ipmap.html");

		open::that("/tmp/ipmap.html").expect("Couldn't open /tmp/ipmap.html");
    });

	let mut mapdata = std::fs::File::create("/tmp/ipmap.data").expect("Couldn't create /tmp/ipmap.data");

	// Set log settings...
	Log::set_opt(Opt::Release);
	Log::remove_old_logs();
	Log::set_level(Level::Notice);

    let mut cap = Device::lookup().unwrap().open().unwrap();

	// Loop for when it is getting packets. Ethan wrote it so I have no idea WTF is in here. Just ignore most of it because it works.
    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
        	// If there is an error, print it. If there is not run stuff.
            Err(value) => println!("IP error {:?}", value),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                	// Run locator with the IP address, which returns Latitude and Longitude.
                    match locator::Locator::get(format!("{}", header.source_addr())) {
                    	Ok(data) => {
                    		mapdata.write_all(format!("\n{}    {}", data.0, data.1).as_bytes()).expect("Couldn't write to /tmp/ipmap.html");
                    		println!("Latitude: {}", data.0);
							println!("Longitude: {}", data.1);
							data
                    	}
                    	// If there was an error, send it to the logs.
                    	Err(error) => {
                    		Log::error(&format!("{}", header.source_addr()));
                    	    Log::error(&format!("{}", error));
                    		(String::from("0.0"), String::from("0.0"))
                    	}
                    };
                }
                Some(_) | None => (),
            },
        }
    }
}
