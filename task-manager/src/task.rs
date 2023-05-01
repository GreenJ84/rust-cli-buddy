use chrono::{DateTime, Local};
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<u64>,
    pub title: String,
    pub description: String,
    pub due_date: Option<DateTime<Local>>,
    pub priority: u32,
    pub status: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        due_date: Option<DateTime<Local>>,
        priority: u32,
        status: String,
    ) -> Self {
        let curr_time = Local::now();
        Self{
            id: None,
            title,
            description,
            due_date,
            priority,
            status,
            created_at: curr_time,
            updated_at: curr_time,
            completed_at: None
        }
    }

    pub fn from_db(
        id: u64,
        title: String,
        description: String,
        due_date: Option<DateTime<Local>>,
        priority: u32,
        status: String,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
        completed_at: Option<DateTime<Local>>
    ) -> Self {
        Self{
            id: Some(id),
            title,
            description,
            due_date,
            priority,
            status,
            created_at: created_at,
            updated_at: updated_at,
            completed_at
        }
    }
}

impl IntoIterator for Task{
    type Item = String;
    type IntoIter = IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter{
        vec![
            self.id.map(|d| d.to_string()).unwrap_or_else(|| "None".to_string()),
            self.title,
            self.description,
            self.due_date.map(|d| format_datetime(d)).unwrap_or_else(|| "None".to_string()),
            self.priority.to_string(),
            self.status,
            format_datetime(self.created_at),
            format_datetime(self.updated_at),
            self.completed_at.map(|d| format_datetime(d)).unwrap_or_else(|| "None".to_string())
        ].into_iter()
    }
}

pub fn format_datetime(dt: DateTime<Local>) -> String {
    dt.format("%A, %B %e, %Y %l:%M %p").to_string()
}
