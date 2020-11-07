extern crate pcap;
extern crate etherparse;

use pcap::Device;
use etherparse::{SlicedPacket, InternetSlice};
fn main() {
    let mut cap = Device::lookup()
                         .unwrap()
                         .open()
                         .unwrap();

    while let Ok(packet) = cap.next() {
        match SlicedPacket::from_ethernet(packet.data) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                match value.ip {
                    Some(InternetSlice::Ipv4(header)) => {
                        println!("{:?}", header.source_addr())
                    }
                    Some(_) | None => (),
                }
            }
        }
    }
}
