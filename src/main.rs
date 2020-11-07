extern crate etherparse;
extern crate pcap;

use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;
use casual_logger::{Level, Log, Opt};

mod locator;

fn main() {
	Log::set_opt(Opt::Release);
	Log::remove_old_logs();
	Log::set_level(Level::Notice);

    let mut cap = Device::lookup().unwrap().open().unwrap();

    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(value) => println!("IP error {:?}", value),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
                    match locator::Locator::get(format!("{}", header.source_addr())) {
                    	Ok(data) => {
                    		println!("IP Address: {:?}", header.source_addr());
                    		println!("Latitude: {}", data.0);
							println!("Longitude: {}", data.1);
							data
                    	}
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
