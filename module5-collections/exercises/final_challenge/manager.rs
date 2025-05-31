use crate::task::{Task, TaskStatus};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String, description: String, due_date: Option<String>) {
        let task = Task::new(self.next_id, title, description, due_date);
        self.tasks.push(task);
        self.next_id += 1;
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks.");
            return;
        }
        for task in &self.tasks {
            println!("{}", task);
        }
    }

    pub fn complete_task(&mut self, id: u32) {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.mark_complete();
                println!("Task #{} marked as completed.", id);
            }
            None => println!("Task not found."),
        }
    }

    pub fn delete_task(&mut self, id: u32) {
        let original_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() < original_len {
            println!("Task #{} deleted.", id);
        } else {
            println!("Task not found.");
        }
    }

    pub fn filter_tasks(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.iter().filter(|task| match task.status {
            TaskStatus::Pending => matches!(status, TaskStatus::Pending),
            TaskStatus::Completed => matches!(status, TaskStatus::Completed),
        }).collect()
    }
}