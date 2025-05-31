mod task;
mod manager;
mod storage;

use manager::TaskManager;
use task::Command;
use std::io::{self, Write};

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Command::from_input(&input);

        match command {
            Command::Add { title, description, due_date } => {
                task_manager.add_task(title, description, due_date);
            }
            Command::List => {
                task_manager.list_tasks();
            }
            Command::Complete { id } => {
                task_manager.complete_task(id);
            }
            Command::Delete { id } => {
                task_manager.delete_task(id);
            }
            Command::Save { filename } => {
                if let Err(e) = storage::save_tasks(&task_manager, &filename) {
                    eprintln!("Error saving: {}", e);
                }
            }
            Command::Load { filename } => {
                match storage::load_tasks(&filename) {
                    Ok(loaded_manager) => task_manager = loaded_manager,
                    Err(e) => eprintln!("Error loading: {}", e),
                }
            }
            Command::Filter { status } => {
                let filtered_tasks = task_manager.filter_tasks(status);
                if filtered_tasks.is_empty() {
                    println!("No tasks found with the specified status.");
                } else {
                    for task in filtered_tasks {
                        println!("{}", task);
                    }
                }
            }
            Command::Quit => {
                println!("Bye!");
                break;
            }
            Command::Unknown => {
                println!("Unknown command.");
            }
        }
    }
}