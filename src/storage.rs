use crate::task::Task;
use std::{
    fs::File,
    io::{self, Read, Write},
    path,
};

pub fn get_temp_file_path() -> path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push("tasks.json");
    path
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let path = get_temp_file_path();

    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path);

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
    let path = get_temp_file_path();

    let json = match serde_json::to_string_pretty(tasks) {
        Ok(j) => j,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
