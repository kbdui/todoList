use crate::service::todo_list_serv;
use crate::init::database;
use anyhow::Result as AnyResult;

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
        "insert" => {
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
        _ => {
            println!("❌ 未知命令: '{}'", order);
            println!("💡 输入 'help' 查看可用命令");
        }
    }
    
    Ok(())

}

// 打印帮助信息
