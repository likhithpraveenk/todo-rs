use crate::task::Task;
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

const FILE: &str = "tasks.json";

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let file = OpenOptions::new().read(true).open(FILE);

    let mut file = match file {
        Ok(f) => f,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let tasks = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let json = match serde_json::to_string_pretty(tasks) {
        Ok(j) => j,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE)?;

    file.write_all(json.as_bytes())?;
    Ok(())
}
