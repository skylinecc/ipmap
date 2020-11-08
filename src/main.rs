extern crate etherparse;
extern crate pcap;
extern crate open;


use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use casual_logger::{Level, Log, Opt};
use serde_json::json;

use std::{
	include_bytes,
	collections::HashSet,
	io::prelude::*,
	thread,
	fs,
	path::Path,
};

mod locator;

fn main() {

	//remove temp files
	if Path::new("/tmp/ipmap.html").is_file() {
	 	fs::remove_file("/tmp/ipmap.html").expect("Couldn't remove /tmp/ipmap.html");
	};

	if Path::new("/tmp/ipmap.json").is_file() {
		fs::remove_file("/tmp/ipmap.json").expect("Couldn't remove /tmp/ipmap.json");
	};

	// Run page.html in another thread.
	thread::spawn(|| {
		let page = include_bytes!("page.html");

		let mut file = std::fs::File::create("/tmp/ipmap.html").expect("Couldn't create /tmp/ipmap.html");
		file.write_all(page).expect("Couldn't write to /tmp/ipmap.html");

		open::that("/tmp/ipmap.html").expect("Couldn't open /tmp/ipmap.html");
    });

	let mut mapdata = std::fs::File::create("/tmp/ipmap.json").expect("Couldn't create /tmp/ipmap.json");
    let mut ip_index = HashSet::new();

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
					let cur_ip = header.source_addr();
					if !ip_index.contains(&cur_ip.to_string()) && !cur_ip.is_private(){
                        ip_index.insert(cur_ip.to_string());
                            // Run locator with the IP address, which returns Latitude and Longitude.
                            match locator::Locator::get(cur_ip.to_string()) {
                    	        Ok(data) => {
							        let json = json!({
								        "location": {
									        "ip": cur_ip,
									        "latitude": data.1,
									        "longitude": data.0,
								    }
                                });
								println!("{}", json);
                    		    mapdata.write_all(format!("\n{}", json).as_bytes()).expect("Couldn't write to /tmp/ipmap.json");
                    	        }
                    	        // If there was an error, send it to the logs.
                    	        Err(error) => {
                    		        Log::error(&cur_ip.to_string());
                    	            Log::error(&error);
                                }
                            }
                    }
                }
                Some(_) | None => (),
            },
        }
    }
}
