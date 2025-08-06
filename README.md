# todo-rs

A simple command-line todo app built with Rust. Tasks are saved in a temporary JSON file (`tasks.json`) within the system's temp directory and can be added, listed, marked as done, or deleted via the CLI.

## Features
- Add tasks via CLI
- List all tasks
- Mark tasks as done
- Delete tasks
- local storage (JSON)

## Dependencies
- [`clap`](https://crates.io/crates/clap) - For command-line argument parsing
- [`serde`](https://crates.io/crates/serde) - For JSON serialization and deserialization
- [`serde_json`](https://crates.io/crates/serde_json) – For reading and writing JSON files
- [`chrono`](https://crates.io/crates/chrono) - For handling date and time
- [`colored`](https://crates.io/crates/colored) - For colored and styled terminal output

## How to Run
1. Clone the repo:
   ```bash
   git clone https://github.com/likhithpraveenk/todo-rs
   cd todo-rs


2. Use the CLI:
    ```
    cargo run

- Available commands: 
    ```
    > add <text>
    > list
    > done <ID>
    > delete <ID>
    > exit


3. Build the project and run it:
   ```bash
   cargo build --release
   ./target/release/todo-rs


## Example Output
    > add Buy Milk
    Added Task: Buy Milk

    Current Tasks:
    1. [•] Buy Milk
    Created  : Aug 05, 2025 at 11:24
    Modified : Aug 05, 2025 at 11:24

    > done 1
    Marked task 1 as done
    1. [✔] Buy Milk
    Created  : Aug 05, 2025 at 11:24
    Modified : Aug 06, 2025 at 20:53

## License

This project is licensed under the [MIT License](LICENSE).
