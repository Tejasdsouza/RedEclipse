use etherparse::PacketHeaders;
use crate::display::display_packet;
use crate::logger::log_packet;

pub fn parse_packet(data: &[u8]) {
    match PacketHeaders::from_ethernet_slice(data) {
        Ok(value) => {
            display_packet(&value);
            log_packet(&value);
        },
        Err(e) => {
            eprintln!("❌ Parse error: {:?}", e);
        }
    }
}