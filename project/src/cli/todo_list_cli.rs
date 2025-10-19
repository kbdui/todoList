use crate::service::todo_list_serv;
use crate::init::database;
use crate::init::config_load;
use anyhow::Result as AnyResult;

// 命令解析与执行
pub fn order_check(order: &str) -> AnyResult<()> {
    // 获取数据库路径并创建连接
    let db_path = config_load::get_config_value("database", Some("path"));
    let db = database::Database::new(&db_path)?;

    // 初始化数据库表结构
    db.initialize_tables()?;

    // 根据命令执行相应操作
    match order {
        "list" => {
            todo_list_serv::show_all_todos(&db)?;
        }
        "help" => {
            print_help();
        }
        "insert" => {
            use crate::data::todo_list::TodoListForm;     // 导入 TodoListForm 结构体，用于后续构造待办事项
            use chrono::Utc;                              // 导入 Utc 获取当前时间

            println!("请输入任务标题:");                   // 提示用户输入任务标题
            let mut title = String::new();                // 创建字符串变量用于存放输入内容
            std::io::stdin().read_line(&mut title)?;      // 从标准输入读取一行到 title
            let title = title.trim().to_string();         // 去掉前后空白，将输入转换为 String 类型

            println!("请输入任务内容:");                   // 提示用户输入任务内容
            let mut description = String::new();          // 创建字符串变量用于存放输入内容
            std::io::stdin().read_line(&mut description)?; // 从标准输入读取一行到 description
            let description = description.trim().to_string(); // 去掉前后空白，将输入转换为 String 类型

            let form = TodoListForm {
                // id 设置为 0，数据库会自动递增生成实际 ID
                id: 0,
                title,
                description: Some(description),
                completed: false,
                begin_time: Utc::now(),
                end_time: None,
                key_message1: None,
                key_message2: None,
                key_message3: None,
            };

            todo_list_serv::add_todo(&db, &form)?;
        }
        "delete" => {
            println!("请输入要删除的任务ID:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id)?;
            let id = id.trim().parse::<i32>()?;
            todo_list_serv::delete_todo(&db, id)?;
        }
        "update" => {
            use crate::data::todo_list::TodoListForm;
            use crate::dao::todo_list_dao;
            
            println!("请输入要更新的任务ID:");
            let mut id = String::new();
            std::io::stdin().read_line(&mut id)?;
            let id = id.trim().parse::<i32>()?;
            
            // 获取现有的 todo
            let conn = db.get_connection();
            let mut todo = todo_list_dao::get_todo_by_id(conn, id)?
                .ok_or_else(|| anyhow::anyhow!("未找到ID为 {} 的任务", id))?;
            
            println!("请输入新的任务标题 (当前: {}):", todo.title);
            let mut title = String::new();
            std::io::stdin().read_line(&mut title)?;
            let title = title.trim();
            if !title.is_empty() {
                todo.title = title.to_string();
            }
            
            println!("请输入新的任务内容 (留空跳过):");
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
fn print_help() {
    println!("📋 可用命令列表:");
    println!("  list  - 显示所有待办事项");
    println!("  help  - 显示此帮助信息");
    println!("  exit  - 退出程序");
    println!("  insert - 添加待办事项");
    println!("  delete - 删除待办事项");
    println!("  update - 更新待办事项");
}