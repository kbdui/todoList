use anyhow::Result as AnyResult;
use crate::dao::todo_list_dao;
use crate::init::database::Database;
use crate::data::todo_list::TodoListForm;
use chrono::{Utc, NaiveDateTime, TimeZone, DateTime};
use std::io::{self, Write};

// 解析时间字符串，支持多种格式
// 支持：YYYY-MM-DD HH:MM:SS, YYYY-MM-DD HH:MM, YYYY-MM-DD HH, YYYY-MM-DD
fn parse_datetime(time_str: &str) -> Result<DateTime<Utc>, String> {
    // 尝试多种格式
    let formats = vec![
        ("%Y-%m-%d %H:%M:%S", ""),           // 完整格式
        ("%Y-%m-%d %H:%M", ":00"),           // 缺少秒
        ("%Y-%m-%d %H", ":00:00"),           // 缺少分秒
        ("%Y-%m-%d", " 00:00:00"),           // 缺少时分秒
    ];

    for (_format, suffix) in formats {
        let full_time_str = format!("{}{}", time_str, suffix);
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(&full_time_str, "%Y-%m-%d %H:%M:%S") {
            return Ok(Utc.from_utc_datetime(&naive_dt));
        }
    }

    Err("时间格式错误".to_string())
}

// 输出所有的事项
pub fn show_all_todos(database: &Database) -> AnyResult<()> {
    let conn = database.get_connection();
    let todos = todo_list_dao::list_todos(conn)?;

    // 打印所有任务到命令行（控制台）
    // 检查 todos 是否为空，并根据结果输出相应的信息
    if todos.is_empty() {
        println!("暂无代办事项");
    } else {
        println!("📋 所有待办事项:");
        println!("{:-<80}", "");
        for (index, todo) in todos.iter().enumerate() {
            let status = if todo.completed { "✅" } else { "⬜" };
            println!("{}. {} [ID: {}] {}", index + 1, status, todo.id, todo.title);
            if let Some(desc) = &todo.description {
                println!("   描述: {}", desc);
            }
            println!("   开始时间: {}", todo.begin_time.format("%Y-%m-%d %H:%M:%S"));
            if let Some(end_time) = &todo.end_time {
                println!("   结束时间: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
            }
            // 显示关键信息（如果存在）
            if let Some(key1) = &todo.key_message1 {
                println!("   关键信息1: {}", key1);
            }
            if let Some(key2) = &todo.key_message2 {
                println!("   关键信息2: {}", key2);
            }
            if let Some(key3) = &todo.key_message3 {
                println!("   关键信息3: {}", key3);
            }
            println!("{:-<80}", "");
        }
    }

    Ok(())
}

pub fn add_todo(database: &Database, form: &TodoListForm) -> AnyResult<()> {
    let conn = database.get_connection();
    todo_list_dao::insert_todo(conn, form)?;
    println!("添加成功");
    Ok(())
}
pub fn delete_todo(database: &Database, id: i32) -> AnyResult<()> {
    let conn = database.get_connection();
    todo_list_dao::delete_todo(conn, id)?;
    println!("删除成功");
    Ok(())
}
pub fn update_todo(database: &Database, form: &TodoListForm) -> AnyResult<()> {
    let conn = database.get_connection();
    todo_list_dao::update_todo(conn, form)?;
    println!("更新成功");

// 创建新的待办事项（交互式输入）
pub fn create_new_todo(database: &Database) -> AnyResult<()> {
    println!("📝 创建新的待办事项");
    println!("提示：标记为 [可选] 的字段可以直接回车跳过\n");

    // 读取标题（必填）
    let title = loop {
        print!("标题 [必填]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let title = input.trim().to_string();
        
        if title.is_empty() {
            println!("❌ 标题不能为空，请重新输入");
            continue;
        }
        break title;
    };

    // 读取描述（可选）
    print!("描述 [可选]: ");
    io::stdout().flush()?;
    let mut description_input = String::new();
    io::stdin().read_line(&mut description_input)?;
    let description = {
        let desc = description_input.trim();
        if desc.is_empty() {
            None
        } else {
            Some(desc.to_string())
        }
    };

    // 读取开始时间（必填）
    let begin_time = loop {
        print!("开始时间 [必填，格式：YYYY-MM-DD [HH[:MM[:SS]]]]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let time_str = input.trim();
        
        if time_str.is_empty() {
            println!("❌ 开始时间不能为空，请重新输入");
            continue;
        }
        
        // 尝试解析时间
        match parse_datetime(time_str) {
            Ok(dt) => break dt,
            Err(_) => {
                println!("❌ 时间格式错误");
                println!("   支持格式：");
                println!("   - 2025-01-01 10:30:00 (完整)");
                println!("   - 2025-01-01 10:30    (秒默认为00)");
                println!("   - 2025-01-01 10       (分秒默认为00:00)");
                println!("   - 2025-01-01          (时分秒默认为00:00:00)");
                continue;
            }
        }
    };

    // 读取结束时间（可选）
    print!("结束时间 [可选，格式：YYYY-MM-DD [HH[:MM[:SS]]]]: ");
    io::stdout().flush()?;
    let mut end_time_input = String::new();
    io::stdin().read_line(&mut end_time_input)?;
    let end_time = loop {
        let time_str = end_time_input.trim();
        if time_str.is_empty() {
            break None
        } else {
            match parse_datetime(time_str) {
                Ok(dt) => break Some(dt),
                Err(_) => {
                    println!("❌ 时间格式错误");
                    println!("   支持格式：");
                    println!("   - 2025-01-01 10:30:00 (完整)");
                    println!("   - 2025-01-01 10:30    (秒默认为00)");
                    println!("   - 2025-01-01 10       (分秒默认为00:00)");
                    println!("   - 2025-01-01          (时分秒默认为00:00:00)");
                    continue;
                }
            }
        }
    };

    // 读取关键信息1（可选）
    print!("关键信息1 [可选]: ");
    io::stdout().flush()?;
    let mut key1_input = String::new();
    io::stdin().read_line(&mut key1_input)?;
    let key_message1 = {
        let key1 = key1_input.trim();
        if key1.is_empty() { None } else { Some(key1.to_string()) }
    };

    let key_message2 = if key_message1.is_none() {
        None
    } else {
        // 读取关键信息2（可选）
        print!("关键信息2 [可选]: ");
        io::stdout().flush()?;
        let mut key2_input = String::new();
        io::stdin().read_line(&mut key2_input)?;
        let key2 = {
            let k2 = key2_input.trim();
            if k2.is_empty() { None } else { Some(k2.to_string()) }
        };

        key2
    };

    // 如果关键信息1为空，则跳过后续关键信息的输入
    let key_message3 = if key_message2.is_none() {
        None
    } else {
        // 读取关键信息3（可选）
        print!("关键信息3 [可选]: ");
        io::stdout().flush()?;
        let mut key3_input = String::new();
        io::stdin().read_line(&mut key3_input)?;
        let key3 = {
            let k3 = key3_input.trim();
            if k3.is_empty() { None } else { Some(k3.to_string()) }
        };

        key3
    };

    // 创建 TodoListForm
    let new_todo = TodoListForm {
        id: 0, // ID 由数据库自动生成，这里的值会被忽略
        title,
        description,
        completed: false, // 新创建的待办事项默认未完成
        begin_time,
        end_time,
        key_message1,
        key_message2,
        key_message3,
    };

    // 插入数据库
    let conn = database.get_connection();
    let todo_id = todo_list_dao::insert_todo(conn, &new_todo)?;

    println!("\n✅ 待办事项创建成功！ID: {}", todo_id);
    println!("   标题: {}", new_todo.title);
    if let Some(desc) = &new_todo.description {
        println!("   描述: {}", desc);
    }
    // 显示关键信息（如果存在）
    if let Some(key1) = &new_todo.key_message1 {
        println!("   关键信息1: {}", key1);
    }
    if let Some(key2) = &new_todo.key_message2 {
        println!("   关键信息2: {}", key2);
    }
    if let Some(key3) = &new_todo.key_message3 {
        println!("   关键信息3: {}", key3);
    }
    Ok(())
}