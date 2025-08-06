use crate::cli::{Cli, Commands};
use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;
use chrono::Local;

pub fn run(cli: Cli) {
    let mut tasks: Vec<Task> = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load tasks: {}", e);
            Vec::new()
        }
    };

    let mut show_tasks = false;

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
                Ok(_) => println!("Added Task: {text}"),
                Err(e) => eprintln!("Couldn't add the task: {e}"),
            };
            show_tasks = true;
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("Warning: No Tasks found!");
            } else {
                println!("Tasks:");
                for task in tasks.iter() {
                    println!("{task}");
                }
            }
        }
        Commands::Done { id } => {
            match tasks.iter_mut().find(|task| task.id == id) {
                Some(task) => {
                    if task.done {
                        println!("Warning: Task {id} is already marked as done")
                    } else {
                        task.done = true;
                        task.modified_at = Local::now();
                        let display_task = task.clone();
                        match save_tasks(&tasks) {
                            Ok(_) => {
                                println!("Marked task {id} as done");
                                println!("{display_task}");
                            }

                            Err(e) => eprintln!("Failed to save task\n {e}"),
                        }
                    }
                }
                None => {
                    eprintln!("No task found with ID: {id}");
                }
            };
        }
        Commands::Delete { id } => {
            if let Some(pos) = tasks.iter().position(|task| task.id == id) {
                let deleted_task = tasks.remove(pos);
                match save_tasks(&tasks) {
                    Ok(_) => {
                        println!("Deleted task {id}");
                        println!("{deleted_task}");
                    }
                    Err(e) => eprintln!("Failed to delete task\n {e}"),
                }
            } else {
                eprintln!("No task found with ID: {id}")
            }
            show_tasks = true;
        }
    }

    if show_tasks {
        println!("\nCurrent Tasks:");
        if tasks.is_empty() {
            println!("(no tasks found)");
        } else {
            for task in tasks.iter() {
                println!("{task}");
            }
        }
    }
    println!();
}
