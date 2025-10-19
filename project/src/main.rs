mod data;
mod dao;
mod cli;
mod service;
mod init;

use std::io::{self, Write};
use cli::help_distribute;
use init::database;
use init::db_json;
use init::config_load;
use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    // 获取数据库路径并创建连接
    let db_path = config_load::get_config_value("database", Some("path"));
    let db = database::Database::new(&db_path)?;

    // 初始化数据库表结构
    db.initialize_tables()?;

    // 获取JSON配置文件路径并初始化
    let json_path = config_load::get_config_value("json", Some("path"));
    let json_config = db_json::JsonConfig::new(&json_path)?;

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
