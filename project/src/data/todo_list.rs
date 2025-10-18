use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoListForm {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub begin_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub key_message1: Option<String>,
    pub key_message2: Option<String>,
    pub key_message3: Option<String>,
}


