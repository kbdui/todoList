use crate::service::help;
use crate::service::switch;
use crate::init::database;
use crate::init::db_json;
use crate::cli::todo_list_cli;
use anyhow::Result as AnyResult;

/// 命令分发中心
/// 这是所有命令的入口，负责：
/// 1. 处理通用命令（help、switch）
/// 2. 根据当前 mode 将其他命令分发到对应的模块
pub fn distribute_command(
    command: &str,
    db: &database::Database,
    json_config: &db_json::JsonConfig,
) -> AnyResult<()> {
    // 首先检查是否为通用命令
    match command {
        "help" => {
            help::print_help(json_config)?;
            return Ok(());
        }
        "switch" => {
            switch::switch_mode(json_config)?;
            return Ok(());
        }
        _ => {
            // 不是通用命令，根据 mode 分发
        }
    }

    // 获取当前模式
    let mode = json_config.get("mode")?;

    // 根据模式分发命令到对应的处理模块
    match mode.as_str() {
        "memo" => {
            // memo 模式：分发给 todo_list_cli 处理
            todo_list_cli::order_check(command, db)?;
        }
        // 未来可以添加更多模式
        // "calendar" => {
        //     calendar_cli::order_check(command, db)?;
        // }
        // "note" => {
        //     note_cli::order_check(command, db)?;
        // }
        _ => {
            println!("❌ 未知模式: '{}'", mode);
            println!("💡 请使用 'switch' 命令切换到有效的模式");
        }
    }

    Ok(())
}

/// 获取当前模式名称（用于显示）
pub fn get_current_mode_name(json_config: &db_json::JsonConfig) -> AnyResult<String> {
    json_config.get("mode")
}

