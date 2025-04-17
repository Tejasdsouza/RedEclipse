use etherparse::{PacketHeaders, NetHeaders, TransportHeader};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn log_packet(headers: &PacketHeaders) {
    let timestamp = Local::now();
    let (src, dst, protocol) = match &headers.net {
        Some(NetHeaders::Ipv4(ip, _)) => (
            Ipv4Addr::from(ip.source).to_string(),
            Ipv4Addr::from(ip.destination).to_string(),
            match &headers.transport {
                Some(TransportHeader::Tcp(_)) => "TCP",
                Some(TransportHeader::Udp(_)) => "UDP",
                Some(_) => "Other",
                None => "Unknown",
            },
        ),
        Some(NetHeaders::Ipv6(ip, _)) => (
            Ipv6Addr::from(ip.source).to_string(),
            Ipv6Addr::from(ip.destination).to_string(),
            match &headers.transport {
                Some(TransportHeader::Tcp(_)) => "TCP",
                Some(TransportHeader::Udp(_)) => "UDP",
                Some(_) => "Other",
                None => "Unknown",
            },
        ),
        _ => ("<no IP>".to_string(), "<no IP>".to_string(), "Unknown"),
    };

    let log_entry = format!(
        "[{}] {} -> {} [{}]\n",
        timestamp.format("%Y-%m-%d %H:%M:%S"),
        src,
        dst,
        protocol
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("packets.log")
        .unwrap();

    let _ = file.write_all(log_entry.as_bytes());
}