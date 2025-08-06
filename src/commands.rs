use crate::cli::{Cli, Commands};
use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;
use chrono::Local;

pub fn run(cli: Cli) {
    let mut tasks: Vec<Task> = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: Failed to load tasks: {}", e);
            Vec::new()
        }
    };

    match cli.command {
        Commands::Add { text } => {
            if text.is_empty() {
                println!("Warning: Task description is empty");
                return;
            }
            let text = text.join(" ");
            let id = tasks.iter().max_by_key(|t| t.id).map_or(1, |t| t.id + 1);
            let task = Task {
                id,
                text: text.clone(),
                done: false,
                created_at: Local::now(),
                modified_at: Local::now(),
            };
            tasks.push(task);
            match save_tasks(&tasks) {
                Ok(_) => println!("Success: Added Task: {text}"),
                Err(e) => eprintln!("Error: Couldn't add the task: {e}"),
            };
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("Warning: No Tasks found!");
            } else {
                println!("Tasks:");
                for task in tasks {
                    let status = if task.done { "✔" } else { " " };
                    println!("{}. [{}] {}", task.id, status, task.text);
                    println!(
                        "Created: {} , Modified: {}",
                        task.created_at.format("%b %d, %Y at %H:%M"),
                        task.modified_at.format("%b %d, %Y at %H:%M")
                    );
                }
            }
        }
        Commands::Done { id } => match tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                if task.done {
                    println!("Warning: Task {id} is already marked as done")
                } else {
                    task.done = true;
                    task.modified_at = Local::now();
                    match save_tasks(&tasks) {
                        Ok(_) => println!("Success: Marked task {id} as done"),
                        Err(e) => eprintln!("Error: Failed to save task\n {e}"),
                    }
                }
            }
            None => {
                eprintln!("Error: No task found with ID: {id}");
            }
        },
        Commands::Delete { id } => {
            let original_len = tasks.len();
            tasks.retain(|task| task.id != id);

            if tasks.len() < original_len {
                match save_tasks(&tasks) {
                    Ok(_) => println!("Success: Deleted task {id}"),
                    Err(e) => eprintln!("Error: Failed to delete task\n {e}"),
                }
            } else {
                eprintln!("Error: No task found with ID: {id}")
            }
        }
    }
}
