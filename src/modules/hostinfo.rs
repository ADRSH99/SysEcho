use whoami;
use colored::*;

pub fn show() {
    println!("{}", "\n User/Host Info".bold().underline().cyan());
    println!("--------------------------");
    println!("{} {}", "User:".green().bold(), whoami::username());
    println!("{} {}", "Desktop Env:".green(), whoami::desktop_env());
    println!("{} {}", "Device's Name".green(),   whoami::devicename());
    println!("{} {}", "Device's Platform".green(),  whoami::platform());
    println!("{} {}", "Device's OS Distro ".green(),  whoami::distro());
    println!("{} {}", "Device's CPU Arch ".green(), whoami::arch());
}
