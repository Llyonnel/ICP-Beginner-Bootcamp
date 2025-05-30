// Starter code for the Rust Task Manager challenge

// Task status enum
#[derive(Debug)]
enum TaskStatus {
    Pending,
    Completed,
}

// Task struct to store task information
#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>, // Consider using a proper date type in your implementation
    status: TaskStatus,
}

// TaskManager to handle operations on tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    
    // Add a new task to the task manager
    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        };
        
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }
    
    // TODO: Implement methods for listing, completing, deleting tasks
    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{:?}", task);
        }
    }

    fn complete_tasks(&mut self, task_id: u32) -> Result<(), String>{
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = TaskStatus::Completed;
            Ok(())
        } else {
            Err(format!("Task with id {} not found", task_id))
        }
    }
    
    fn delete_task(&mut self, task_id: u32) -> Result<(), String> {
        if let Some(i) = self.tasks.iter().position(|t| t.id == task_id) {
            self.tasks.remove(i);
            Ok(())
        } else {
            Err(format!("Task with id {} not found", task_id))
        }
    }
    
    // TODO: Implement methods for saving to and loading from a file

    // TODO: Implement methods for filtering tasks
    fn filter_tasks(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.iter().filter(|task| match task.status {
            TaskStatus::Pending => matches!(status, TaskStatus::Pending),
            TaskStatus::Completed => matches!(status, TaskStatus::Completed),
        }).collect()
    }
}

// Command enum to represent user commands
enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Quit,
    Unknown,
}

fn main() {
    // Initialize task manager
    let mut task_manager = TaskManager::new();
    
    // Main application loop
    loop {
        // TODO: Get user command
        println!("Enter command (add, list, complete, delete, save, load, quit):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim().to_lowercase();
        
        // TODO: Process command
        match command.as_str() {
            "add" => {
                println!("Enter task title:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).expect("Failed to read line");
                
                println!("Enter task description:");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).expect("Failed to read line");
                
                println!("Enter due date (optional):");
                let mut due_date = String::new();
                std::io::stdin().read_line(&mut due_date).expect("Failed to read line");
                
                task_manager.add_task(
                    title.trim().to_string(),
                    description.trim().to_string(),
                    if due_date.trim().is_empty() { None } else { Some(due_date.trim().to_string()) },
                );
                println!("Task added successfully.");
            },
            "list" => {
                task_manager.list_tasks();
            },
            "complete" => {
                println!("Enter task ID to complete:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).expect("Failed to read line");
                task_manager.complete_tasks(id.trim().parse().unwrap_or(0))
                    .expect("Failed to complete task");
                println!("Task completed successfully.");  
            },
            "delete" => {
                println!("Enter task ID to delete:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).expect("Failed to read line");
                task_manager.delete_task(id.trim().parse().unwrap_or(0))
                    .expect("Failed to delete task");
                println!("Task deleted successfully.");
            },
            "quit" => {break;},
            _ => {
                println!("Unknown command. Please try again.");
            },
        };
    }
}
