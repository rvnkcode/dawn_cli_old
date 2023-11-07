use chrono::{DateTime, Local};

pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed_at: Option<DateTime<Local>>,
    pub note: Option<String>,
    // pub is_completed: bool,
}
