use std::fmt;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub due_date: Option<String>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, title: String, description: String, due_date: Option<String>) -> Self {
        Task {
            id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        }
    }

    pub fn mark_complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} [{}] - {} \n{}{}",
            self.id,
            match self.status {
                TaskStatus::Pending => "Pending",
                TaskStatus::Completed => "Done",
            },
            self.title,
            self.description,
            match &self.due_date {
                Some(date) => format!("\nDue: {}", date),
                None => "".to_string(),
            }
        )
    }
}

pub enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    Filter { status: TaskStatus },
    Quit,
    Unknown,
}

impl Command {
    pub fn from_input(input: &str) -> Command {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Command::Unknown;
        }

        match parts[0].to_lowercase().as_str() {
            "add" => {
                let title = parts.get(1).unwrap_or(&"").to_string();
                let description = parts.get(2).unwrap_or(&"").to_string();
                let due_date = parts.get(3).map(|s| s.to_string());
                Command::Add { title, description, due_date }
            }
            "list" => Command::List,
            "complete" => {
                if let Some(id) = parts.get(1).and_then(|s| s.parse().ok()) {
                    Command::Complete { id }
                } else {
                    Command::Unknown
                }
            }
            "delete" => {
                if let Some(id) = parts.get(1).and_then(|s| s.parse().ok()) {
                    Command::Delete { id }
                } else {
                    Command::Unknown
                }
            }
            "save" => {
                let filename = parts.get(1).unwrap_or(&"tasks.json").to_string();
                Command::Save { filename }
            }
            "load" => {
                let filename = parts.get(1).unwrap_or(&"tasks.json").to_string();
                Command::Load { filename }
            }
            "filter" => {
                if parts.len() < 2 {
                    return Command::Unknown;
                }
                let status = match parts[1].to_lowercase().as_str() {
                    "pending" => TaskStatus::Pending,
                    "completed" => TaskStatus::Completed,
                    _ => return Command::Unknown,
                };
                Command::Filter { status }
            }
            "quit" | "exit" => Command::Quit,
            _ => Command::Unknown,
        }
    }
}