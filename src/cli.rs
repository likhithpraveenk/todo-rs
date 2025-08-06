use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo-rs")]
#[command(version = "0.1.0")]
#[command(about = "A simple command-line todo app written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task description
        text: Vec<String>,
    },
    /// List all tasks
    List,
    /// Mark a task as done
    Done {
        /// ID of the task to mark as done
        id: usize,
    },
    /// Delete a task
    Delete {
        /// ID of the task to delete
        id: usize,
    },
}
