use crate::data::todo_list::TodoListForm;
use crate::data::reminder::ReminderType;
use crate::init::database::Database;
use crate::init::db_json::JsonConfig;
use crate::init::db_json_content::{ReminderConfig, ReminderRule};
use crate::dao::todo_list_dao;
use crate::dao::reminder_dao;
use crate::service::logger::Logger;
use chrono::Utc;
use anyhow::Result as AnyResult;

pub struct ReminderService;

impl ReminderService {
    /// 检查所有待办事项的提醒
    pub fn check_and_notify(
        db: &Database,
        json_config: &JsonConfig,
    ) -> AnyResult<Vec<String>> {
        let mut notifications = Vec::new();
        
        // 读取配置
        let reminder_config = Self::get_reminder_config(json_config)?;
        
        if !reminder_config.enabled {
            return Ok(notifications);
        }
        
        // 获取所有未完成的待办事项
        let conn = db.get_connection();
        let todos = todo_list_dao::list_todos(conn)?;
        let uncompleted: Vec<_> = todos.into_iter()
            .filter(|t| !t.completed)
            .collect();
        
        let now = Utc::now();
        
        for todo in uncompleted {
            // begin_time 总是存在的（非 Option 类型）
            let begin_time = todo.begin_time;
            // 检查每个提醒规则
            for rule in &reminder_config.rules {
                if let Some(notification) = Self::check_rule(
                    &todo,
                    begin_time,
                    now,
                    rule,
                    db,
                )? {
                    notifications.push(notification.clone());
                    Logger::log_reminder(&notification);
                }
            }
        }
        
        Ok(notifications)
    }
    
    /// 获取提醒配置
    fn get_reminder_config(json_config: &JsonConfig) -> AnyResult<ReminderConfig> {
        // 尝试读取整个配置
        match json_config.get_value("reminder") {
            Ok(value) => {
                let config: ReminderConfig = serde_json::from_value(value)?;
                Ok(config)
            }
            Err(_) => {
                // 如果没有配置，返回默认值
                Ok(ReminderConfig::default())
            }
        }
    }
    
    /// 检查单个提醒规则
    fn check_rule(
        todo: &TodoListForm,
        begin_time: chrono::DateTime<chrono::Utc>,
        now: chrono::DateTime<chrono::Utc>,
        rule: &ReminderRule,
        db: &Database,
    ) -> AnyResult<Option<String>> {
        let reminder_type = match rule.rule_type.as_str() {
            "before_start" => {
                let seconds = rule.seconds_before.unwrap_or(0);
                if seconds == 86400 {
                    ReminderType::OneDayBefore
                } else if seconds == 3600 {
                    ReminderType::OneHourBefore
                } else {
                    return Ok(None);
                }
            }
            "overdue" => ReminderType::Overdue,
            _ => return Ok(None),
        };
        
        // 检查是否需要提醒
        let should_remind = match &reminder_type {
            ReminderType::OneDayBefore => {
                let diff = begin_time.signed_duration_since(now);
                diff.num_seconds() > 0 
                    && diff.num_seconds() <= 86400
                    && !Self::has_been_notified(db, todo.id, &reminder_type)?
            }
            ReminderType::OneHourBefore => {
                let diff = begin_time.signed_duration_since(now);
                diff.num_seconds() > 0 
                    && diff.num_seconds() <= 3600
                    && !Self::has_been_notified(db, todo.id, &reminder_type)?
            }
            ReminderType::Overdue => {
                begin_time < now 
                    && !Self::has_been_notified(db, todo.id, &reminder_type)?
            }
        };
        
        if should_remind {
            // 记录提醒历史
            Self::record_notification(db, todo.id, &reminder_type)?;
            
            // 生成提醒消息
            let message = rule.message_template
                .replace("{title}", &todo.title)
                .replace("{id}", &todo.id.to_string());
            
            return Ok(Some(message));
        }
        
        Ok(None)
    }
    
    /// 检查是否已经发送过提醒
    fn has_been_notified(
        db: &Database,
        todo_id: i32,
        reminder_type: &ReminderType,
    ) -> AnyResult<bool> {
        let conn = db.get_connection();
        Ok(reminder_dao::has_been_notified(conn, todo_id, reminder_type)?)
    }
    
    /// 记录提醒历史
    fn record_notification(
        db: &Database,
        todo_id: i32,
        reminder_type: &ReminderType,
    ) -> AnyResult<()> {
        let conn = db.get_connection();
        Ok(reminder_dao::record_notification(conn, todo_id, reminder_type)?)
    }
    
    /// 显示提醒历史
    pub fn show_history(db: &Database) -> AnyResult<()> {
        let conn = db.get_connection();
        let reminders = reminder_dao::get_all_reminders(conn)?;
        
        if reminders.is_empty() {
            println!("📋 暂无提醒历史");
            return Ok(());
        }
        
        println!("\n📋 提醒历史记录（最近100条）:");
        println!("{}", "=".repeat(80));
        
        for reminder in reminders {
            let type_label = match ReminderType::from_string(&reminder.reminder_type) {
                Some(ReminderType::OneDayBefore) => "提前1天",
                Some(ReminderType::OneHourBefore) => "提前1小时",
                Some(ReminderType::Overdue) => "已逾期",
                None => "未知类型",
            };
            
            println!("  [ID:{}] 任务ID:{} | {} | {}",
                reminder.id,
                reminder.todo_id,
                type_label,
                reminder.reminder_time.format("%Y-%m-%d %H:%M:%S")
            );
        }
        
        println!("{}", "=".repeat(80));
        println!();
        
        Ok(())
    }
    
    /// 清理旧的提醒历史
    pub fn cleanup_old_history(db: &Database, days: i64) -> AnyResult<()> {
        let conn = db.get_connection();
        let deleted = reminder_dao::cleanup_old_history(conn, days)?;
        
        println!("✓ 已清理 {} 天前的提醒历史，共删除 {} 条记录", days, deleted);
        Logger::log("INFO", &format!("清理了{}天前的{}条提醒历史", days, deleted));
        
        Ok(())
    }
}

