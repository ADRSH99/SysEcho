
use colored::*;
use sysinfo::{Networks};
use get_if_addrs::get_if_addrs;

pub fn show() {
    let networks = Networks::new_with_refreshed_list();

    println!("{}", "\n Network Info".bold().underline().magenta());

    // Section 1: Traffic Stats
    for (iface, data) in &networks {
        println!("{} {}", "Interface:".bright_cyan(), iface);
        println!("  {} {} B", "Received:".bright_green(), data.total_received());
        println!("  {} {} B", "Transmitted:".bright_green(), data.total_transmitted());
        println!(); // spacing
    }

    // Section 2: IP Addresses
    println!("{}", "🔗 Interface IPs".bold().underline().blue());
    for iface in get_if_addrs().unwrap() {
        println!("{} {}", "Interface:".bright_cyan(), iface.name);
        println!("  {} {}", "IP:".bright_green(), iface.ip());
        println!(); // spacing
    }
}
