use pulse::timer::Timer;
use pulse::cli::{Cli, Command};

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Ls => {
            println!("listing shizz");
        },
        _ => {}
    }

    Ok(())
}
