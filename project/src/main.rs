mod data;
mod dao;
mod cli;
mod service;
mod init;
mod runner;

use std::io::{self, Write};
use std::env;
use cli::help_distribute;
use init::{database, db_json, config_load};
use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 检查是否是提醒检查模式（由系统定时任务调用）
    if args.len() > 1 && args[1] == "--check-reminders" {
        return runner::reminder::run_check_mode();
    }
    
    // 正常的交互式模式
    run_interactive_mode()
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
    
    // 启动时检查一次提醒（可选功能）
    // runner::reminder::check_on_startup(&db, &json_config)?;

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
            println!("已退出程序");
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
