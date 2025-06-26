
use colored::*;
use sysinfo::{System};

pub fn show() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "\n Running Processes".bold().underline().yellow());

    for (pid, process) in sys.processes() {
        println!(
            "{} [{}]  {}  {}",
            "•".cyan(),
            pid.to_string().bright_yellow(),
            process.name().bright_white().bold(),
            format!("{:?}", process.disk_usage()).dimmed()
        );
    }
}
