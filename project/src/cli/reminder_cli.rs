use crate::init::database::Database;
use crate::init::db_json::JsonConfig;
use crate::service::reminder_serv;
use anyhow::Result as AnyResult;
use std::io::{self, Write};

/// Reminder 模式专用命令解析与执行
pub fn order_check(order: &str, db: &Database, json_config: &JsonConfig) -> AnyResult<()> {
    match order {
        "reminder" => {
            toggle_reminder(json_config)?;
        }
        "reminder-status" => {
            show_reminder_status(json_config)?;
        }
        "reminder-history" => {
            reminder_serv::ReminderService::show_history(db)?;
        }
        "reminder-cleanup" => {
            cleanup_reminder_history(db)?;
        }
        "test-reminder" => {
            test_reminder(db, json_config)?;
        }
        _ => {
            println!("❌ 未知命令: '{}'", order);
            println!("💡 输入 'help' 查看可用命令");
        }
    }
    
    Ok(())
}

/// 切换提醒功能开关
fn toggle_reminder(json_config: &JsonConfig) -> AnyResult<()> {
    // 读取当前状态
    let current_enabled = match json_config.get_value("reminder") {
        Ok(value) => {
            value.get("enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(true)
        }
        Err(_) => true,
    };
    
    println!("📋 提醒功能设置");
    println!("{}", "=".repeat(60));
    println!("当前状态: {}", if current_enabled { "✅ 已启用" } else { "❌ 已禁用" });
    println!();
    println!("请选择:");
    println!("  1. 启用提醒功能");
    println!("  2. 禁用提醒功能");
    println!("  3. 取消");
    println!();
    
    print!("请输入选项 (1-3): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();
    
    match choice {
        "1" => {
            let was_enabled = current_enabled;
            update_reminder_enabled(json_config, true)?;
            println!("✅ 提醒功能已启用");
            
            // 如果之前是禁用状态，现在启用了，询问是否重启
            if !was_enabled {
                println!();
                println!("💡 提醒功能已启用，建议重启程序以便立即生效");
                println!("   程序重启时会自动设置定时任务（如果尚未设置）");
                println!();
                print!("是否现在重启程序？(Y/N): ");
                io::stdout().flush()?;
                
                let mut restart_input = String::new();
                io::stdin().read_line(&mut restart_input)?;
                
                if restart_input.trim().eq_ignore_ascii_case("y") {
                    println!();
                    println!("🔄 正在重启程序...");
                    println!("👋 再见！");
                    std::process::exit(0);
                } else {
                    println!("💡 您也可以手动退出程序后重新启动");
                }
            }
        }
        "2" => {
            update_reminder_enabled(json_config, false)?;
            println!("❌ 提醒功能已禁用");
            println!("💡 定时任务不会被删除，您可以稍后重新启用");
        }
        "3" => {
            println!("操作已取消");
        }
        _ => {
            println!("⚠️  无效的选项");
        }
    }
    
    Ok(())
}

/// 更新提醒功能状态
fn update_reminder_enabled(json_config: &JsonConfig, enabled: bool) -> AnyResult<()> {
    let mut config = match json_config.get_value("reminder") {
        Ok(value) => {
            serde_json::from_value::<crate::init::db_json_content::ReminderConfig>(value)?
        }
        Err(_) => crate::init::db_json_content::ReminderConfig::default(),
    };
    
    config.enabled = enabled;
    let value = serde_json::to_value(config)?;
    json_config.set_value("reminder", value)?;
    
    Ok(())
}

/// 显示提醒功能状态
fn show_reminder_status(json_config: &JsonConfig) -> AnyResult<()> {
    let config = match json_config.get_value("reminder") {
        Ok(value) => {
            serde_json::from_value::<crate::init::db_json_content::ReminderConfig>(value)?
        }
        Err(_) => crate::init::db_json_content::ReminderConfig::default(),
    };
    
    println!("\n📊 提醒功能状态");
    println!("{}", "=".repeat(60));
    println!("功能状态: {}", if config.enabled { "✅ 已启用" } else { "❌ 已禁用" });
    println!("通知类型: {}", config.notification_type);
    println!();
    println!("提醒规则:");
    for (i, rule) in config.rules.iter().enumerate() {
        println!("  {}. {} - {}", 
            i + 1, 
            rule.rule_type,
            if let Some(seconds) = rule.seconds_before {
                format!("提前{}秒", seconds)
            } else {
                "逾期提醒".to_string()
            }
        );
    }
    println!("{}", "=".repeat(60));
    println!();
    
    Ok(())
}

/// 清理旧提醒历史
fn cleanup_reminder_history(db: &Database) -> AnyResult<()> {
    println!("📋 清理提醒历史");
    println!("{}", "=".repeat(60));
    println!("请输入要保留的天数（将删除更早的记录）:");
    println!("  例如: 30 (保留最近30天的记录)");
    println!();
    
    print!("天数: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let days: i64 = input.trim().parse()?;
    
    if days < 1 {
        println!("⚠️  天数必须大于0");
        return Ok(());
    }
    
    println!();
    print!("确认要删除 {} 天前的提醒历史吗？(y/n): ", days);
    io::stdout().flush()?;
    
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;
    
    if confirm.trim().eq_ignore_ascii_case("y") {
        reminder_serv::ReminderService::cleanup_old_history(db, days)?;
    } else {
        println!("操作已取消");
    }
    
    Ok(())
}

/// 测试提醒功能
fn test_reminder(db: &Database, json_config: &JsonConfig) -> AnyResult<()> {
    println!("🔔 正在测试提醒功能...");
    println!("{}", "=".repeat(60));
    
    let notifications = reminder_serv::ReminderService::check_and_notify(db, json_config)?;
    
    if notifications.is_empty() {
        println!("✓ 当前没有需要提醒的事项");
    } else {
        println!("✓ 找到 {} 条提醒:", notifications.len());
        for notification in notifications {
            println!("  • {}", notification);
        }
    }
    
    println!("{}", "=".repeat(60));
    println!();
    
    Ok(())
}

