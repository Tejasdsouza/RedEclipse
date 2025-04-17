# :red_circle: RedEclipse LAN Analyzer

RedEclipse is a simple, Rust-powered LAN packet analyzer designed to inspect and color-code live network traffic — just like Wireshark, but lighter and terminal-based.

---

## :sparkles: Features

- :white_check_mark: Live packet capture from any LAN interface
- :rainbow: Color-coded output for TCP, UDP and other protocols
- :bar_chart: Real-time stats panel (total packets, bytes, protocol counts)
- :floppy_disk: Full packet logs saved to `packets.log` with timestamped files

---

## :inbox_tray: Requirements

### :white_check_mark: Software You Need

| Tool            | Why it's Needed                          | Link |
|-----------------|-------------------------------------------|------|
| Rust            | To build and run the analyzer             | [Install Rust](https://www.rust-lang.org/tools/install) |
| Npcap Runtime   | For capturing packets (driver)            | [Npcap Installer](https://npcap.com/#download) |
| Npcap SDK       | Required to build the project with `pcap` | [Npcap SDK](https://npcap.com/#download) |

> :small_orange_diamond: **Note:** Some users may also need to manually copy `wpcap.dll` from `C:\Windows\System32\` to your `target/debug` or `target/release` folder if the app doesn't launch properly.

---

## :gear: Toolchain Target

This project requires the **`x86_64-pc-windows-msvc`** Rust target.

To ensure it's installed:

If not then run these commands:

```bash
rustup install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
rustup override set stable-x86_64-pc-windows-msvc

---

## :wrench: Setting Up Build Config (Important!)

To let Cargo find the `wpcap.lib` file from your Npcap SDK:

1. Open file `.cargo/config.toml` in your project root.
2. Add the following:

```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-L", "your/path/to/npcap-sdk/lib/x64"] 

---
## :rocket: Getting Started

```bash
git clone https://github.com/TejasDsouza7/RedEclipse.git
cd RedEclipse
cargo build
cargo run
