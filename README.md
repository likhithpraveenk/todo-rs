# todo-rs

## Project Description
A simple command-line todo app built with Rust. Tasks are saved locally in a JSON file and can be added, listed, marked as done, or deleted via the CLI.

## Features
- Add tasks via CLI
- List all tasks
- Mark tasks as done
- Delete tasks
- Persistent local storage (JSON)

## Dependencies
- [`serde`](https://crates.io/crates/serde) – For JSON serialization and deserialization
- [`chrono`](https://crates.io/crates/chrono) – For handling date and time

## How to Run
1. Clone the repo:
   ```bash
   git clone https://github.com/likhithpraveenk/todo-rs
   cd todo-rs

2. Build the project:
   ```bash
   cargo build --release

3. Use the CLI:
    ```bash
    ./target/release/todo-rs add "Buy milk"
    ./target/release/todo-rs list
    ./target/release/todo-rs done 1
    ./target/release/todo-rs delete 1

## Data Storage
All tasks are saved in a local file called `tasks.json` in the project directory.

## Example Output
    Tasks:
    1. [✔] Buy milk
    Created: Aug 06, 2025 at 14:02 , Modified: Aug 06, 2025 at 14:03

## Licence

This project is licensed under the [MIT License](LICENSE).