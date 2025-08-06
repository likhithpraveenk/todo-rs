mod cli;
mod commands;
mod storage;
mod task;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    commands::run(cli);
}
