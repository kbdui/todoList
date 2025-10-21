use crate::service::todo_list_serv;
use crate::service::note_serv;
use crate::init::database;
use anyhow::Result as AnyResult;
use crate::data::note::NoteForm;
use chrono::Utc;

// // 命令定义结构
// pub struct CommandInfo {
//     pub name: &'static str,
//     #[allow(dead_code)]
//     pub description: &'static str,
// }
// // 所有可用命令列表
// pub const AVAILABLE_COMMANDS: &[CommandInfo] = &[
//     CommandInfo { name: "list", description: "显示所有待办事项" },
//     CommandInfo { name: "new", description: "创建新的待办事项" },
//     CommandInfo { name: "delete", description: "删除待办事项" },
//     CommandInfo { name: "update", description: "更新待办事项" },
//     CommandInfo { name: "toggle", description: "切换待办事项完成状态" },
//     CommandInfo { name: "note", description: "管理待办事项的笔记" },
//     CommandInfo { name: "help", description: "显示帮助信息" },
//     CommandInfo { name: "exit", description: "退出程序" },
// ];

// Memo 模式专用命令解析与执行
pub fn order_check(order: &str, db: &database::Database) -> AnyResult<()> {

    // 根据命令执行相应操作
    match order {
        "list" => {
            todo_list_serv::show_all_todos(&db)?;
        }
        "new" => {
            todo_list_serv::create_new_todo(&db)?;
        }
        "delete" => {
            println!("请输入要删除的任务ID:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id)?;
            let id = id.trim().parse::<i32>()?;
            todo_list_serv::delete_todo(&db, id)?;
        }
        "update" => {
            use crate::dao::todo_list_dao;
            println!("请输入要更新的任务ID:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id)?;
            let id = id.trim().parse::<i32>()?;

            // 获取现有的 todo
            let conn = db.get_connection();
            let mut todo = todo_list_dao::get_todo_by_id(conn, id)?
                .ok_or_else(|| anyhow::anyhow!("未找到ID为 {} 的任务", id))?;

            // 获取新的标题
            println!("请输入新的任务标题 (当前: {}):", todo.title);
            let mut title = String::new();
            std::io::stdin().read_line(&mut title)?;
            let title = title.trim();
            if !title.is_empty() {
                todo.title = title.to_string();
            }

            // 获取新的描述
            println!("请输入新的任务描述 (留空跳过):");
            let mut description = String::new();
            std::io::stdin().read_line(&mut description)?;
            let description = description.trim();
            if !description.is_empty() {
                todo.description = Some(description.to_string());
            }

            todo_list_serv::update_todo(&db, &todo)?;
        }
        "toggle" => {
            println!("请输入要切换完成状态的待办事项ID:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id)?;
            let id = id.trim().parse::<i32>()?;
            todo_list_serv::toggle_completed(&db, id)?;
        }
        "note" => {
            handle_note_command(&db)?;
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

/// 处理笔记管理命令
fn handle_note_command(db: &database::Database) -> AnyResult<()> {
    use crate::dao::todo_list_dao;

    // 第一步：选择 todo 项目
    println!("\n📋 请先选择要管理笔记的待办事项:");
    println!("输入 'list' 查看所有待办事项，或直接输入待办事项ID:");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    // 如果输入 list，显示所有 todo
    if input == "list" {
        todo_list_serv::show_all_todos(db)?;
        println!("\n请输入要管理的待办事项ID:");
        let mut id_input = String::new();
        std::io::stdin().read_line(&mut id_input)?;
        let todo_id = id_input.trim().parse::<i32>()?;
        manage_notes_for_todo(db, todo_id)?;
    } else {
        let todo_id = input.parse::<i32>()?;
        
        // 验证 todo 是否存在
        let conn = db.get_connection();
        let todo = todo_list_dao::get_todo_by_id(conn, todo_id)?
            .ok_or_else(|| anyhow::anyhow!("未找到ID为 {} 的待办事项", todo_id))?;
        
        println!("\n✅ 已选择待办事项: {}", todo.title);
        manage_notes_for_todo(db, todo_id)?;
    }

    Ok(())
}

/// 管理特定 todo 项目的笔记
fn manage_notes_for_todo(db: &database::Database, todo_id: i32) -> AnyResult<()> {
    loop {
        println!("\n📝 笔记管理 (待办事项 ID: {})", todo_id);
        println!("可用操作:");
        println!("  list   - 查看所有笔记");
        println!("  add    - 添加新笔记");
        println!("  update - 更新笔记");
        println!("  delete - 删除笔记");
        println!("  back   - 返回主菜单");
        print!("\n请输入操作: ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut operation = String::new();
        std::io::stdin().read_line(&mut operation)?;
        let operation = operation.trim();

        match operation {
            "list" => {
                note_serv::show_notes_for_todo(db, todo_id)?;
            }
            "add" => {
                println!("\n请输入笔记标题:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title)?;
                let title = title.trim().to_string();

                println!("请输入笔记内容:");
                let mut content = String::new();
                std::io::stdin().read_line(&mut content)?;
                let content = content.trim().to_string();

                println!("请输入优先级 (留空跳过):");
                let mut priority = String::new();
                std::io::stdin().read_line(&mut priority)?;
                let priority = priority.trim();
                let priority = if priority.is_empty() {
                    None
                } else {
                    Some(priority.to_string())
                };

                println!("请输入标签 (留空跳过):");
                let mut tag = String::new();
                std::io::stdin().read_line(&mut tag)?;
                let tag = tag.trim();
                let tag = if tag.is_empty() {
                    None
                } else {
                    Some(tag.to_string())
                };

                let note = NoteForm {
                    id: 0,
                    todo_id,
                    note_title: title,
                    note_content: content,
                    note_time: Utc::now(),
                    noter: None,
                    note_type: None,
                    note_status: None,
                    note_tag: tag,
                    note_priority: priority,
                };

                note_serv::add_note(db, &note)?;
            }
            "update" => {
                use crate::dao::note_dao;
                
                println!("\n请输入要更新的笔记ID:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id)?;
                let id = id.trim().parse::<i32>()?;

                // 获取现有笔记
                let conn = db.get_connection();
                let mut note = note_dao::get_note_by_id(conn, id)?
                    .ok_or_else(|| anyhow::anyhow!("未找到ID为 {} 的笔记", id))?;

                println!("请输入新的标题 (当前: {}, 留空跳过):", note.note_title);
                let mut title = String::new();
                std::io::stdin().read_line(&mut title)?;
                let title = title.trim();
                if !title.is_empty() {
                    note.note_title = title.to_string();
                }

                println!("请输入新的内容 (留空跳过):");
                let mut content = String::new();
                std::io::stdin().read_line(&mut content)?;
                let content = content.trim();
                if !content.is_empty() {
                    note.note_content = content.to_string();
                }

                note.note_time = Utc::now();
                note_serv::update_note(db, &note)?;
            }
            "delete" => {
                println!("\n请输入要删除的笔记ID:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id)?;
                let id = id.trim().parse::<i32>()?;

                note_serv::delete_note(db, id)?;
            }
            "back" => {
                break;
            }
            _ => {
                println!("❌ 未知操作: '{}'", operation);
            }
        }
    }

    Ok(())
}

// 打印帮助信息
