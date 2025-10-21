use crate::init::database::Database;
use crate::init::db_json::JsonConfig;
use crate::init::config_reset;
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
        "reminder-type" => {
            set_notification_type(json_config)?;
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
    
    // 读取当前间隔
    let current_interval = match json_config.get_value("reminder") {
        Ok(value) => {
            value.get("check_interval_minutes")
                .and_then(|v| v.as_u64())
                .unwrap_or(15) as u32
        }
        Err(_) => 15,
    };
    
    println!("📋 提醒功能设置");
    println!("{}", "=".repeat(60));
    println!("当前状态: {}", if current_enabled { "✅ 已启用" } else { "❌ 已禁用" });
    println!("当前间隔: 每 {} 分钟", current_interval);
    println!();
    println!("请选择:");
    println!("  1. 启用提醒功能");
    println!("  2. 禁用提醒功能");
    println!("  3. 修改检查间隔（提醒功能保持启用状态）");
    println!("  4. 取消");
    println!();
    
    print!("请输入选项 (1-4): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();
    
    match choice {
        "1" => {
            let was_enabled = current_enabled;
            
            // 询问检查间隔时间
            println!();
            println!("⏱️  请设置提醒检查间隔（分钟）:");
            println!("  推荐值: 15 (每15分钟检查一次)");
            println!("  范围: 1-1440 (最多24小时)");
            println!();
            print!("请输入间隔分钟数 [默认15]: ");
            io::stdout().flush()?;
            
            let mut interval_input = String::new();
            io::stdin().read_line(&mut interval_input)?;
            let interval_str = interval_input.trim();
            
            let interval_minutes = if interval_str.is_empty() {
                15  // 默认值
            } else {
                match interval_str.parse::<u32>() {
                    Ok(val) if val >= 1 && val <= 1440 => val,
                    _ => {
                        println!("⚠️  无效的输入，使用默认值 15 分钟");
                        15
                    }
                }
            };
            
            update_reminder_config(json_config, true, interval_minutes)?;
            println!("✅ 提醒功能已启用");
            println!("⏱️  检查间隔: 每 {} 分钟", interval_minutes);
            
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
            // 读取当前配置，保持间隔时间不变
            update_reminder_config(json_config, false, current_interval)?;
            println!("❌ 提醒功能已禁用");
            println!("💡 下次启动程序时，会自动删除定时任务");
        }
        "3" => {
            // 修改检查间隔（保持启用状态）
            println!();
            println!("⏱️  修改提醒检查间隔");
            println!("{}", "=".repeat(60));
            println!("当前间隔: 每 {} 分钟", current_interval);
            println!();
            println!("请设置新的检查间隔（分钟）:");
            println!("  推荐值: 15 (每15分钟检查一次)");
            println!("  范围: 1-1440 (最多24小时)");
            println!();
            print!("请输入间隔分钟数 [默认15]: ");
            io::stdout().flush()?;
            
            let mut interval_input = String::new();
            io::stdin().read_line(&mut interval_input)?;
            let interval_str = interval_input.trim();
            
            let new_interval = if interval_str.is_empty() {
                15  // 默认值
            } else {
                match interval_str.parse::<u32>() {
                    Ok(val) if val >= 1 && val <= 1440 => val,
                    _ => {
                        println!("⚠️  无效的输入，使用默认值 15 分钟");
                        15
                    }
                }
            };
            
            // 检查是否有变化
            if new_interval == current_interval {
                println!("💡 间隔未改变，无需更新");
                return Ok(());
            }
            
            // 更新配置（保持启用状态）
            update_reminder_config(json_config, current_enabled, new_interval)?;
            
            // 标记配置已改变，提示需要更新任务
            config_reset::mark_reminder_config_changed(json_config)?;
            
            println!();
            println!("✅ 检查间隔已更新为: 每 {} 分钟", new_interval);
            println!();
            println!("⚠️  重要提示：需要重启程序才能应用新的间隔时间");
            println!("   程序重启时会自动更新任务计划程序中的设置");
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
                println!("💡 请记得手动重启程序以应用新设置");
            }
        }
        "4" => {
            println!("操作已取消");
        }
        _ => {
            println!("⚠️  无效的选项");
        }
    }
    
    Ok(())
}

/// 更新提醒功能配置
fn update_reminder_config(json_config: &JsonConfig, enabled: bool, interval_minutes: u32) -> AnyResult<()> {
    let mut config = match json_config.get_value("reminder") {
        Ok(value) => {
            serde_json::from_value::<crate::init::db_json_content::ReminderConfig>(value)?
        }
        Err(_) => crate::init::db_json_content::ReminderConfig::default(),
    };
    
    config.enabled = enabled;
    config.check_interval_minutes = interval_minutes;
    let value = serde_json::to_value(config)?;
    json_config.set_value("reminder", value)?;
    
    Ok(())
}

/// 设置通知类型
fn set_notification_type(json_config: &JsonConfig) -> AnyResult<()> {
    let current_type = match json_config.get_value("reminder") {
        Ok(value) => {
            value.get("notification_type")
                .and_then(|v| v.as_str())
                .unwrap_or("console")
                .to_string()
        }
        Err(_) => "console".to_string(),
    };
    
    println!("\n🔔 通知类型设置");
    println!("{}", "=".repeat(60));
    println!("当前类型: {}", match current_type.as_str() {
        "console" => "📟 控制台通知（仅显示在终端）",
        "windows" => "🪟 Windows 通知（系统托盘弹窗）",
        "both" => "🔔 双重通知（控制台 + Windows）",
        _ => &current_type,
    });
    println!();
    println!("请选择通知类型:");
    println!("  1. 📟 控制台通知 - 仅在终端显示（适合开发/调试）");
    println!("  2. 🪟 Windows 通知 - 系统托盘弹窗（适合后台定时任务）");
    println!("  3. 🔔 双重通知 - 同时使用两种方式");
    println!("  4. 取消");
    println!();
    println!("💡 提示: 定时任务建议使用 Windows 通知，避免弹出控制台窗口");
    println!();
    
    print!("请输入选项 (1-4): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();
    
    let new_type = match choice {
        "1" => "console",
        "2" => "windows",
        "3" => "both",
        "4" => {
            println!("操作已取消");
            return Ok(());
        }
        _ => {
            println!("⚠️  无效的选项");
            return Ok(());
        }
    };
    
    // 更新配置
    let mut config = match json_config.get_value("reminder") {
        Ok(value) => {
            serde_json::from_value::<crate::init::db_json_content::ReminderConfig>(value)?
        }
        Err(_) => crate::init::db_json_content::ReminderConfig::default(),
    };
    
    config.notification_type = new_type.to_string();
    let value = serde_json::to_value(config)?;
    json_config.set_value("reminder", value)?;
    
    println!();
    println!("✅ 通知类型已更新为: {}", match new_type {
        "console" => "📟 控制台通知",
        "windows" => "🪟 Windows 通知",
        "both" => "🔔 双重通知",
        _ => new_type,
    });
    println!("💡 提示: 您可以使用 'test-reminder' 命令测试通知效果");
    println!();
    
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
    println!("检查间隔: 每 {} 分钟", config.check_interval_minutes);
    println!("通知类型: {}", match config.notification_type.as_str() {
        "console" => "📟 控制台通知",
        "windows" => "🪟 Windows 通知",
        "both" => "🔔 双重通知",
        _ => &config.notification_type,
    });
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
    
    if days < 0 {
        println!("⚠️  天数必须大于等于0");
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

