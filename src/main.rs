use pulse::{
    Pulse,
    cli::{Cli, Command},
};

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut pulse = Pulse { projects: vec![] };
    pulse.add_project("beer")?;
    pulse.find_project_mut("beer")
        .unwrap()
        .add_task("get groceries");

    match &cli.command {
        Command::Ls => {
            Table::new(pulse.projects);
        },
        Command::Use { name } => {
        },
        Command::Track { task, notes }  => {
        },
        Command::Task(test) => {
        },
        Command::Break => {
        },
        _ => {}
    }

    Ok(())
}
