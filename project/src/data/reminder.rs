use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderHistory {
    pub id: i32,
    pub todo_id: i32,
    pub reminder_time: DateTime<Utc>,
    pub reminder_type: String,
    pub notified: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReminderType {
    OneDayBefore,
    OneHourBefore,
    Overdue,
}

impl ReminderType {
    pub fn to_string(&self) -> String {
        match self {
            ReminderType::OneDayBefore => "1_day_before".to_string(),
            ReminderType::OneHourBefore => "1_hour_before".to_string(),
            ReminderType::Overdue => "overdue".to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "1_day_before" => Some(ReminderType::OneDayBefore),
            "1_hour_before" => Some(ReminderType::OneHourBefore),
            "overdue" => Some(ReminderType::Overdue),
            _ => None,
        }
    }
}

