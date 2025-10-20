use crate::service::review_serv;
use crate::init::database;
use anyhow::Result as AnyResult;

// // Review 模式专用命令列表
// pub struct CommandInfo {
//     pub name: &'static str,
//     #[allow(dead_code)]
//     pub description: &'static str,
// }

// // 所有可用命令列表
// pub const AVAILABLE_COMMANDS: &[CommandInfo] = &[
//     CommandInfo { name: "stats", description: "显示整体统计信息" },
//     CommandInfo { name: "completed", description: "显示已完成的待办事项" },
//     CommandInfo { name: "pending", description: "显示未完成的待办事项" },
//     CommandInfo { name: "notes", description: "显示最近的笔记" },
// ];

/// Review 模式命令解析与执行
pub fn order_check(order: &str, db: &database::Database) -> AnyResult<()> {
    match order {
        "stats" => {
            review_serv::show_statistics(db)?;
        }
        "completed" => {
            review_serv::show_completed_todos(db)?;
        }
        "pending" => {
            review_serv::show_pending_todos(db)?;
        }
        "notes" => {
            // 默认显示最近 10 条笔记
            review_serv::show_recent_notes(db, 10)?;
        }
        _ => {
            println!("❌ 未知命令: '{}'", order);
            println!("💡 输入 'help' 查看可用命令");
            // println!("\n可用命令: {}", 
            //     AVAILABLE_COMMANDS.iter()
            //         .map(|c| c.name)
            //         .collect::<Vec<_>>()
            //         .join(", ")
            // );
        }
    }
    
    Ok(())
}

