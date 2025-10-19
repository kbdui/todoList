use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteForm {
    pub id: i32,
    pub todo_id: i32,  // 关联的 todo 项目 ID
    pub note_title: String,
    pub note_content: String,
    pub note_time: DateTime<Utc>,
    pub noter: Option<String>,
    pub note_type: Option<String>,
    pub note_status: Option<String>,
    pub note_tag: Option<String>,
    pub note_priority: Option<String>,   // 笔记优先级
}