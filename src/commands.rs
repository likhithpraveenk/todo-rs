use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;
use chrono::Local;
use std::{env, usize};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_help();
        return;
    }

    let mut tasks: Vec<Task> = match load_tasks() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: Failed to load tasks: {}", e);
            Vec::new()
        }
    };

    match args[1].as_str() {
        "add" => {
            if args.len() >= 3 {
                let text = args[2..].join(" ");
                let id = tasks.len() + 1;
                let task = Task {
                    id,
                    text: text.clone(),
                    done: false,
                    created_at: Local::now(),
                    modified_at: Local::now(),
                };
                tasks.push(task);
                match save_tasks(&tasks) {
                    Ok(_) => println!("Success: Added Task: {}", text),
                    Err(e) => eprintln!("Error: Couldn't add the task: {e}"),
                };
            } else {
                println!("Warning: Did not provide any task to be added");
                print_help();
            }
        }
        "list" => {
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
        "done" => {
            if args.len() >= 3 {
                let id = match args[2].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Error: Invalid ID: '{}'", args[2]);
                        return;
                    }
                };

                let mut found = false;

                for task in &mut tasks {
                    if task.id == id {
                        task.done = true;
                        task.modified_at = Local::now();
                        found = true;
                        break;
                    }
                }

                if found {
                    match save_tasks(&tasks) {
                        Ok(_) => println!("Success: Marked task {id} as done"),
                        Err(e) => eprintln!("Error: Failed to save task: {e}"),
                    }
                } else {
                    eprintln!("Error: No task found with ID: {id}")
                }
            } else {
                println!("Warning: Provide the ID of a task to mark as done");
                print_help();
            }
        }
        "delete" => {
            if args.len() >= 3 {
                let id = match args[2].parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Error: Invalid ID: '{}'", args[2]);
                        return;
                    }
                };

                let original_len = tasks.len();
                tasks.retain(|task| task.id != id);

                if tasks.len() < original_len {
                    match save_tasks(&tasks) {
                        Ok(_) => println!("Success: Deleted task {id}"),
                        Err(e) => eprintln!("Error: Failed to delete task: {e}"),
                    }
                } else {
                    eprintln!("Error: No task found with ID: {id}")
                }
            } else {
                println!("Warning: Provide the ID of a task to be deleted");
                print_help();
            }
        }
        _ => {
            println!("Warning: Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("todo-rs: A simple command-line todo app built with Rust");
    println!();
    println!("Usage:");
    println!("  todo-rs add \"Task description\"  Add a new task");
    println!("  todo-rs list                    List all tasks");
    println!("  todo-rs done <id>               Mark a task as done");
    println!("  todo-rs delete <id>             Delete a task");
    println!();
}
