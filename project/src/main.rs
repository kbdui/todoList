mod data;
mod dao;
mod cli;
mod service;
mod init;

use std::io::{self, Write};
use std::env;
use cli::help_distribute;
use init::database;
use init::db_json;
use init::config_load;
use service::reminder_serv;
use service::notifier;
use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 检查是否是提醒检查模式
    if args.len() > 1 && args[1] == "--check-reminders" {
        return run_reminder_check();
    }
    
    // 正常的交互式模式
    run_interactive_mode()
}

/// 提醒检查模式（由系统定时任务调用）
fn run_reminder_check() -> AnyResult<()> {
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

/// 交互式模式（正常使用）
fn run_interactive_mode() -> AnyResult<()> {
    // 获取数据库路径并创建连接
    let db_path = config_load::get_config_value("database", Some("path"));
    let db = database::Database::new(&db_path)?;

    // 初始化数据库表结构
    db.initialize_tables()?;

    // 获取JSON配置文件路径并初始化
    let json_path = config_load::get_config_value("json", Some("path"));
    let json_config = db_json::JsonConfig::new(&json_path)?;
    
    // 启动时检查一次提醒
    check_reminders_on_startup(&db, &json_config)?;

    println!("=== Todo List 管理系统 ===");
    println!("输入命令进行操作，输入 'exit' 退出程序");
    println!("💡 输入 'help' 查看可用命令");
    println!();

    loop {
        // 打印提示符
        print!("> ");
        io::stdout().flush().unwrap();

        // 读取用户输入
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("读取输入失败: {}", e);
            continue;
        }

        // 去除首尾空白字符
        let command = input.trim();

        // 检查是否退出
        if command == "exit" {
            println!("再见！");
            break;
        }

        // 跳过空输入
        if command.is_empty() {
            continue;
        }

        // 执行命令（通过命令分发中心）
        if let Err(e) = help_distribute::distribute_command(command, &db, &json_config) {
            eprintln!("执行命令时出错: {}", e);
        }

        println!(); // 空行分隔
    }

    Ok(())
}

/// 启动时检查提醒
fn check_reminders_on_startup(db: &database::Database, json_config: &db_json::JsonConfig) -> AnyResult<()> {
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
