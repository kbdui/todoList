use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub struct Logger;

impl Logger {
    /// 记录提醒日志
    pub fn log_reminder(message: &str) {
        let log_file = "database/reminder.log";
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_message = format!("[{}] {}\n", timestamp, message);
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
        {
            let _ = file.write_all(log_message.as_bytes());
        }
    }
    
    /// 记录一般日志
    pub fn log(level: &str, message: &str) {
        let log_file = "database/app.log";
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_message = format!("[{}] [{}] {}\n", timestamp, level, message);
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
        {
            let _ = file.write_all(log_message.as_bytes());
        }
    }
}

