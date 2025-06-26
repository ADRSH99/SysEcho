use colored::*;
use sysinfo::{System, Disks, Components};

pub fn show() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "\n  System Info".bold().underline().blue());

    // RAM and Swap Info
    println!("{}", " Memory Usage".bold().cyan());
    println!("  {} {} KB", "Total RAM:".green(), sys.total_memory());
    println!("  {} {} KB", "Used RAM :".green(), sys.used_memory());
    println!("  {} {} KB", "Total Swap:".green(), sys.total_swap());
    println!("  {} {} KB", "Used Swap :".green(), sys.used_swap());

    // System Info
    println!("\n{}", " System Details".bold().cyan());
    println!("  {} {:?}", "Name:".yellow(), System::name());
    println!("  {} {:?}", "Kernel:".yellow(), System::kernel_version());
    println!("  {} {:?}", "OS Version:".yellow(), System::os_version());
    println!("  {} {:?}", "Host:".yellow(), System::host_name());
    println!("  {} {}", "CPUs:".yellow(), sys.cpus().len());

    // Disk Info
    println!("\n{}", " Disk Info".bold().cyan());
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!(
            "  {} {} | Type: {:?} | Mount: {:?}",
            "Disk:".bright_green(),
            disk.name().to_string_lossy(),
            disk.kind(),
            disk.mount_point()
        );
    }

    // Temperature Info
    println!("\n{}", " Components".bold().cyan());
    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!(
            "  {}: {:.1}°C",
            component.label().bright_magenta(),
            component.temperature()
        );
    }
}
