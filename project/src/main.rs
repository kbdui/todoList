mod data;
mod dao;
mod cli;
mod service;
mod init;

use std::io::{self, Write};
use cli::todo_list_cli;

fn main() {
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

        // 执行命令
        if let Err(e) = todo_list_cli::order_check(command) {
            eprintln!("执行命令时出错: {}", e);
        }

        println!(); // 空行分隔
    }
}
