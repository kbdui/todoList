/// 提醒功能的运行器
/// 
/// 该模块负责协调提醒功能的运行，包括：
/// - 定时任务模式：由系统调度器调用，执行提醒检查
/// - 启动检查模式：在程序启动时检查并显示提醒

use crate::init::{database, db_json, config_load};
use crate::service::{reminder_serv, notifier};
use anyhow::Result as AnyResult;

/// 提醒检查模式（由系统定时任务调用）
/// 
/// 该函数会：
/// 1. 初始化数据库和配置
/// 2. 检查所有需要提醒的待办事项
/// 3. 根据配置的通知类型发送提醒
/// 
/// 使用方式：
/// ```bash
/// project.exe --check-reminders
/// ```
pub fn run_check_mode() -> AnyResult<()> {
    // 初始化数据库
    let db_path = config_load::get_config_value("database", Some("path"));
    let db = database::Database::new(&db_path)?;
    db.initialize_tables()?;
    
    // 初始化 JSON 配置
    let json_path = config_load::get_config_value("json", Some("path"));
    let json_config = db_json::JsonConfig::new(&json_path)?;
    
    // 检查提醒
    let notifications = reminder_serv::ReminderService::check_and_notify(&db, &json_config)?;
    
    if notifications.is_empty() {
        println!("✓ 没有需要提醒的事项");
        return Ok(());
    }
    
    // 读取通知类型配置
    let notification_type = match json_config.get_value("reminder") {
        Ok(value) => {
            value.get("notification_type")
                .and_then(|v| v.as_str())
                .unwrap_or("console")
                .to_string()
        }
        Err(_) => "console".to_string(),
    };
    
    // 发送通知
    for notification in notifications {
        notifier::Notifier::send(&notification, &notification_type)?;
        println!("✓ 已发送提醒: {}", notification);
    }
    
    Ok(())
}

/// 启动时检查提醒
/// 
/// 在程序启动时检查是否有需要提醒的待办事项，
/// 如果有，则在控制台以醒目方式显示
/// 
/// # 参数
/// * `db` - 数据库连接
/// * `json_config` - JSON 配置
#[allow(dead_code)]
pub fn check_on_startup(
    db: &database::Database,
    json_config: &db_json::JsonConfig,
) -> AnyResult<()> {
    let notifications = reminder_serv::ReminderService::check_and_notify(db, json_config)?;
    
    if !notifications.is_empty() {
        println!("\n{}", "⏰".repeat(30));
        println!("📋 您有 {} 条待办事项需要注意：", notifications.len());
        println!("{}", "⏰".repeat(30));
        for notification in notifications {
            println!("  • {}", notification);
        }
        println!("{}\n", "⏰".repeat(30));
    }
    
    Ok(())
}

