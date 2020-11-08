extern crate etherparse;
extern crate pcap;
extern crate open;
extern crate pnet;

use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use casual_logger::{Level, Log, Opt};
use serde_json::json;

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

	// Set log settings
	Log::set_opt(Opt::Release);
	Log::remove_old_logs();
	Log::set_level(Level::Notice);

    let mut cap = Device::lookup().unwrap().open().unwrap();

	//loop through each packet in the capture interface as an iterator until it returns an error
    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(value) => println!("IP error {:?}", value),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
					// Run locator with the IP address, which returns Latitude and Longitude.
					if header.source_addr().is_private() {
						println!("{}", header.source_addr());
						continue;
					}
                    match locator::Locator::get(format!("{}", header.source_addr())) {
                    	Ok(data) => {
							let json = json!({
								"location": {
									"ip": header.source_addr(),
									"latitude": data.1,
									"longitude": data.0,
								}
                    	    });
                    	  //  println!("{}", json);
                    		mapdata.write_all(format!("\n{}", json).as_bytes()).expect("Couldn't write to /tmp/ipmap.html");
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
