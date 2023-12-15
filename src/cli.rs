use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// List all projects and their active timers.
    Ls,

    /// Put focus on particular project.
    Use { name: String },

    /// Track time in the project currently being focussed on.
    Track {
        /// The name of the task to track.
        task: String,

        /// Additional notes describing the activity.
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// Manage tasks.
    Task(TaskArgs),

    /// Take a well-deserved break.
    Break,
}

#[derive(Args)]
pub struct TaskArgs {
    #[command(subcommand)]
    command: TaskCommand,
}

#[derive(Subcommand)]
pub enum TaskCommand {
    /// List all tasks in the current project.
    Ls,
    /// Remove a task from the current project.
    Rm { name: String },
    /// Add a task to the current project.
    Add { name: String },
    /// Edit a task in the current project.
    Edit { name: String },
}
