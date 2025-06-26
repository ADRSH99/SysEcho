mod modules;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sysecho", version = "1.0", about = "ineededitsoimadeit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show user and host details
    Host,

    /// Show system summary (RAM, CPU, uptime, etc.)
    Sys,

    /// Show network interfaces and usage
    Net,

    /// Show battery status
    Battery,

    /// Show everything (all sections)
    All,

    ///Show all processes
    Proc,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Host => modules::hostinfo::show(),
        Commands::Sys => modules::systeminfo::show(),
        Commands::Net => modules::network::show(),
        Commands::Battery => modules::battery::show(),
        Commands::Proc => modules::process::show(),
        Commands::All => {
            modules::hostinfo::show();
            modules::systeminfo::show();
            modules::network::show();
            modules::battery::show();
            modules::process::show();
        }
    }
}
