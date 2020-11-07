extern crate etherparse;
extern crate pcap;

use etherparse::{InternetSlice, SlicedPacket};
use pcap::Device;

mod locator;

fn main() {
    let mut cap = Device::lookup().unwrap().open().unwrap();

    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(value) => println!("IP error {:?}", value),
            Ok(value) => match value.ip {
                Some(InternetSlice::Ipv4(header)) => {
					println!("IP Address: {:?}", header.source_addr());
					
                    match locator::Locator::get(format!("{}", header.source_addr())) {
                    	Ok(data) => {
                    		println!("Latitude: {}", data.0);
							println!("Longitude: {}", data.1);
							data
                    	}
                    	Err(error) => {
							eprintln!("ERROR: {}", error);
                    		(String::from("0.0"), String::from("0.0"))
                    	}
                    };
                }
                Some(_) | None => (),
            },
        }
    }
}
