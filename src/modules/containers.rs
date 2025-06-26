use colored::*;
use duct::cmd;
use std::process::Command;

pub fn show(){
    if Command::new("docker").arg("--version").output().is_err() {
        eprintln!("{}", "Docker not found. Skipping container stats. Check fix on repo".red());
        return;
    }

    let output = cmd!(
        "docker",
        "stats",
        "--no-stream",
        "--format",
        "{{.Container}} {{.Name}} {{.CPUPerc}} {{.MemUsage}}"
    )
    .read();

    match output {
        Ok(raw) => {
            println!("\n{}", "Docker Containers:".bold().underline());
            println!("{:<15} {:<20} {:<10} {}", "CONTAINER ID", "NAME", "CPU", "MEMORY");

            for line in raw.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let container_id = parts[0];
                    let name = parts[1];
                    let cpu = parts[2];
                    let mem = parts[3..].join(" ");
                    println!(
                        "{:<15} {:<20} {:<10} {}",
                        container_id.cyan(),
                        name.yellow(),
                        cpu.green(),
                        mem
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Failed to fetch docker stats: {}", e).red());
        }
    }


}