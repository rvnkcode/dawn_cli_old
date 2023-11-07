use chrono::{DateTime, Local};

pub struct Todo {
    pub id: u32,
    pub is_completed: bool,
    pub title: String,
    pub completed_at: Option<DateTime<Local>>,
    pub note: Option<String>,
    pub created_at: Option<DateTime<Local>>,
}
