mod parser;
mod display;
mod logger;
mod stats;

use pcap::{Device, Capture};
use std::io::{self, Write};
use parser::parse_packet;
use stats::Stats;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n🔴 RedEclipse LAN Analyzer Started 🔴\n");

    let devices = Device::list().expect("Failed to list devices");
    if devices.is_empty() {
        eprintln!("⚠️ No devices found!");
        return;
    }

    for (i, dev) in devices.iter().enumerate() {
        println!("{}: {} ({:?})", i, dev.name, dev.desc);
    }

    print!("\nSelect device number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice: usize = input.trim().parse().unwrap_or_else(|_| {
        eprintln!("❌ Invalid input.");
        std::process::exit(1);
    });

    if choice >= devices.len() {
        eprintln!("❌ Invalid device index.");
        return;
    }

    let device = &devices[choice];
    println!("\n✅ Using device: {}\n", device.name);

    let mut cap = Capture::from_device(device.name.as_str()).unwrap()
        .promisc(true)
        .snaplen(65535)
        .timeout(1000)
        .open().unwrap();

    let stats = Arc::new(Mutex::new(Stats::new()));
    let stats_clone = Arc::clone(&stats);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        stats_clone.lock().unwrap().display();
    });

    while let Ok(packet) = cap.next_packet() {
        let data = packet.data;
        stats.lock().unwrap().update(&data);
        parse_packet(data);
    }
}
