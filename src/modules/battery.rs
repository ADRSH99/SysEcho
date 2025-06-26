
use colored::*;
use battery::Manager;
use battery::units::ratio::percent;

pub fn show() {
    let manager = Manager::new().expect("Failed to create battery manager");

    println!("{}", "\n Battery Info".bold().underline().red());
    for (i, maybe_battery) in manager.batteries().unwrap().enumerate() {
        let battery = maybe_battery.unwrap();
        println!("{} #{}", "Battery".bright_white().bold(), i + 1);
        println!("  {} {:?}", "State:".bright_yellow(), battery.state());
        println!(
            "  {} {:.2}%",
            "Charge:".bright_yellow(),
            battery.state_of_charge().get::<percent>()
        );
        println!(
            "  {} {:?}",
            "Time to Full:".bright_yellow(),
            battery.time_to_full()
        );
        println!(""); // spacing
    }
}
