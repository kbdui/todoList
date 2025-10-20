use anyhow::Result as AnyResult;
use crate::dao::todo_list_dao;
use crate::dao::note_dao;
use crate::init::database::Database;
use chrono::Utc;

/// 解析优先级字符串为数字（用于排序）
/// 高优先级返回较小的数字，这样排序时会排在前面
fn parse_priority(priority: &Option<String>) -> i32 {
    match priority {
        None => 999, // 无优先级排在最后
        Some(p) => {
            let p_lower = p.to_lowercase();
            match p_lower.as_str() {
                "高" | "high" | "1" | "urgent" | "紧急" => 1,
                "中" | "medium" | "2" | "normal" | "普通" => 2,
                "低" | "low" | "3" | "minor" | "次要" => 3,
                _ => {
                    // 尝试解析为数字
                    p.parse::<i32>().unwrap_or(999)
                }
            }
        }
    }
}

/// 显示整体统计信息
pub fn show_statistics(database: &Database) -> AnyResult<()> {
    let conn = database.get_connection();
    let todos = todo_list_dao::list_todos(conn)?;

    if todos.is_empty() {
        println!("📊 暂无任何待办事项");
        return Ok(());
    }

    // 统计数据
    let total = todos.len();
    let completed = todos.iter().filter(|t| t.completed).count();
    let pending = total - completed;
    let completion_rate = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // 统计笔记总数
    let mut total_notes = 0;
    for todo in &todos {
        let notes = note_dao::list_notes_by_todo_id(conn, todo.id)?;
        total_notes += notes.len();
    }

    println!("\n📊 整体统计");
    println!("{:=<80}", "");
    println!("总待办事项数: {}", total);
    println!("已完成: {} ({}%)", completed, completion_rate as i32);
    println!("未完成: {}", pending);
    println!("笔记总数: {}", total_notes);
    println!("{:=<80}", "");

    Ok(())
}

/// 显示已完成的待办事项
pub fn show_completed_todos(database: &Database) -> AnyResult<()> {
    let conn = database.get_connection();
    let todos = todo_list_dao::list_todos(conn)?;

    let completed_todos: Vec<_> = todos.iter().filter(|t| t.completed).collect();

    if completed_todos.is_empty() {
        println!("✅ 暂无已完成的待办事项");
        return Ok(());
    }

    println!("\n✅ 已完成的待办事项 ({} 项):", completed_todos.len());
    println!("{:=<80}", "");
    
    for (index, todo) in completed_todos.iter().enumerate() {
        println!("{}. [ID: {}] {}", index + 1, todo.id, todo.title);
        if let Some(desc) = &todo.description {
            println!("   描述: {}", desc);
        }
        println!("   开始时间: {}", todo.begin_time.format("%Y-%m-%d %H:%M:%S"));
        if let Some(end_time) = &todo.end_time {
            println!("   结束时间: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
            
            // 计算用时
            let duration = end_time.signed_duration_since(todo.begin_time);
            let days = duration.num_days();
            let hours = duration.num_hours() % 24;
            let minutes = duration.num_minutes() % 60;
            
            if days > 0 {
                println!("   用时: {} 天 {} 小时 {} 分钟", days, hours, minutes);
            } else if hours > 0 {
                println!("   用时: {} 小时 {} 分钟", hours, minutes);
            } else {
                println!("   用时: {} 分钟", minutes);
            }
        }
        
        // 显示笔记数量
        let notes = note_dao::list_notes_by_todo_id(conn, todo.id)?;
        if !notes.is_empty() {
            println!("   📝 笔记数: {}", notes.len());
        }
        
        println!("{:-<80}", "");
    }

    Ok(())
}

/// 显示未完成的待办事项
pub fn show_pending_todos(database: &Database) -> AnyResult<()> {
    let conn = database.get_connection();
    let todos = todo_list_dao::list_todos(conn)?;

    let pending_todos: Vec<_> = todos.iter().filter(|t| !t.completed).collect();

    if pending_todos.is_empty() {
        println!("⬜ 暂无未完成的待办事项");
        return Ok(());
    }

    let now = Utc::now();
    println!("\n⬜ 未完成的待办事项 ({} 项):", pending_todos.len());
    println!("{:=<80}", "");
    
    for (index, todo) in pending_todos.iter().enumerate() {
        println!("{}. [ID: {}] {}", index + 1, todo.id, todo.title);
        if let Some(desc) = &todo.description {
            println!("   描述: {}", desc);
        }
        println!("   开始时间: {}", todo.begin_time.format("%Y-%m-%d %H:%M:%S"));
        
        // 计算已经过去的时间
        let elapsed = now.signed_duration_since(todo.begin_time);
        let days = elapsed.num_days();
        let hours = elapsed.num_hours() % 24;
        
        if days > 0 {
            println!("   已过去: {} 天 {} 小时", days, hours);
        } else if hours > 0 {
            println!("   已过去: {} 小时", hours);
        } else {
            println!("   已过去: {} 分钟", elapsed.num_minutes());
        }
        
        if let Some(end_time) = &todo.end_time {
            // 检查是否超期
            if now > *end_time {
                let overdue = now.signed_duration_since(*end_time);
                let overdue_days = overdue.num_days();
                println!("   ⚠️  已超期 {} 天", overdue_days);
            } else {
                println!("   截止时间: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
            }
        }
        
        // 显示笔记数量
        let notes = note_dao::list_notes_by_todo_id(conn, todo.id)?;
        if !notes.is_empty() {
            println!("   📝 笔记数: {}", notes.len());
        }
        
        println!("{:-<80}", "");
    }

    Ok(())
}

/// 显示最近的笔记
pub fn show_recent_notes(database: &Database, limit: usize) -> AnyResult<()> {
    let conn = database.get_connection();
    let todos = todo_list_dao::list_todos(conn)?;

    if todos.is_empty() {
        println!("📝 暂无任何笔记");
        return Ok(());
    }

    // 收集所有笔记（包含todo_id）
    let mut all_notes = Vec::new();
    for todo in &todos {
        let notes = note_dao::list_notes_by_todo_id(conn, todo.id)?;
        for note in notes {
            all_notes.push((todo.id, todo.title.clone(), note));
        }
    }

    if all_notes.is_empty() {
        println!("📝 暂无任何笔记");
        return Ok(());
    }

    // 首先按待办事项ID排序，其次按优先级排序
    all_notes.sort_by(|a, b| {
        // 首先比较 todo_id
        let todo_cmp = a.0.cmp(&b.0);
        if todo_cmp != std::cmp::Ordering::Equal {
            return todo_cmp;
        }
        // todo_id 相同时，按优先级排序
        parse_priority(&a.2.note_priority).cmp(&parse_priority(&b.2.note_priority))
    });

    // 取前 limit 个
    let recent_notes: Vec<_> = all_notes.iter().take(limit).collect();

    println!("\n📝 笔记列表 (最多显示 {} 条，按待办ID和优先级排序):", limit);
    println!("{:=<80}", "");
    
    for (index, (todo_id, todo_title, note)) in recent_notes.iter().enumerate() {
        println!("{}. [笔记ID: {}] {}", index + 1, note.id, note.note_title);
        println!("   所属待办: [ID: {}] {}", todo_id, todo_title);
        println!("   内容: {}", note.note_content);
        println!("   时间: {}", note.note_time.format("%Y-%m-%d %H:%M:%S"));
        if let Some(ref priority) = note.note_priority {
            println!("   优先级: {}", priority);
        }
        if let Some(ref tag) = note.note_tag {
            println!("   标签: {}", tag);
        }
        println!("{:-<80}", "");
    }

    Ok(())
}

