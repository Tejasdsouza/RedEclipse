use std::collections::HashMap;

pub struct Stats {
    pub total_packets: usize,
    pub protocol_counts: HashMap<String, usize>,
    pub total_bytes: usize,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            total_packets: 0,
            protocol_counts: HashMap::new(),
            total_bytes: 0,
        }
    }

    pub fn update(&mut self, packet: &[u8]) {
        self.total_packets += 1;
        self.total_bytes += packet.len();

        let proto = if packet.len() > 23 {
            match packet[23] {
                6 => "TCP",
                17 => "UDP",
                _ => "Other",
            }
        } else {
            "Unknown"
        };

        *self.protocol_counts.entry(proto.to_string()).or_insert(0) += 1;
    }

    pub fn display(&self) {
        println!("\n📊 Real-Time Stats:");
        println!("Total Packets: {}", self.total_packets);
        println!("Total Bytes: {} bytes", self.total_bytes);
        for (proto, count) in &self.protocol_counts {
            println!("{} Packets: {}", proto, count);
        }
        println!("-------------------------\n");
    }
}
