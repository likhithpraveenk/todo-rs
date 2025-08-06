mod cli;
mod commands;
mod storage;
mod task;

use std::io::{self, Write};

use clap::Parser;
use cli::Cli;

fn main() {
    let separator = "=".repeat(20);
    println!("{separator} todo-rs {separator}");
    println!("Available commands: add <text>, list, done <ID>, delete <ID>, quit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error: Failed to read input");
            continue;
        }

        let trimmed = input.trim();
        let exit_cmds = ["exit", "quit", "q"];
        if exit_cmds
            .iter()
            .any(|cmd| trimmed.eq_ignore_ascii_case(cmd))
        {
            println!("Exiting...");
            break;
        }
        let args = std::iter::once("todo-rs")
            .chain(trimmed.split_whitespace())
            .collect::<Vec<_>>();
        match Cli::try_parse_from(args) {
            Ok(cli) => commands::run(cli),
            Err(e) => eprintln!("{e}"),
        }
    }
}
