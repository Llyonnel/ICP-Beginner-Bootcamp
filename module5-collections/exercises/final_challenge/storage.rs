use crate::manager::TaskManager;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

pub fn save_tasks(manager: &TaskManager, filename: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(manager)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_tasks(filename: &str) -> Result<TaskManager, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let manager: TaskManager = serde_json::from_str(&content)?;
    Ok(manager)
}