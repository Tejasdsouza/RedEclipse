use etherparse::{PacketHeaders, NetHeaders, TransportHeader};
use colored::*;
use chrono::Local;
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn display_packet(headers: &PacketHeaders) {
    let timestamp = Local::now();
    let formatted_time = timestamp.format("[%H:%M:%S]").to_string().cyan();

    let (src, dst) = match &headers.net {
        Some(NetHeaders::Ipv4(ip, _)) => (
            Ipv4Addr::from(ip.source).to_string(),
            Ipv4Addr::from(ip.destination).to_string(),
        ),
        Some(NetHeaders::Ipv6(ip, _)) => (
            Ipv6Addr::from(ip.source).to_string(),
            Ipv6Addr::from(ip.destination).to_string(),
        ),
        _ => ("<no IP>".to_string(), "<no IP>".to_string()),
    };

    let protocol = match &headers.transport {
        Some(TransportHeader::Tcp(_)) => "TCP".green(),
        Some(TransportHeader::Udp(_)) => "UDP".blue(),
        Some(_) => "Other".magenta(),
        None => "Unknown".dimmed(),
    };

    println!(
        "{} {} {} ➜ {}",
        formatted_time,
        protocol,
        src,
        dst
    );
}