use anyhow::Result as AnyResult;
use crate::data::note::NoteForm;
use crate::dao::note_dao;
use crate::init::database::Database;

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

/// 显示某个 todo 项目的所有笔记
pub fn show_notes_for_todo(database: &Database, todo_id: i32) -> AnyResult<()> {
    let conn = database.get_connection();
    let mut notes = note_dao::list_notes_by_todo_id(conn, todo_id)?;

    if notes.is_empty() {
        println!("📝 该待办事项暂无笔记");
    } else {
        // 按优先级排序
        notes.sort_by(|a, b| {
            parse_priority(&a.note_priority).cmp(&parse_priority(&b.note_priority))
        });
        
        println!("\n📝 笔记列表 (待办事项ID: {}):", todo_id);
        println!("{}", "=".repeat(80));
        for note in &notes {
            println!("ID: {}", note.id);
            println!("标题: {}", note.note_title);
            println!("内容: {}", note.note_content);
            println!("时间: {}", note.note_time.format("%Y-%m-%d %H:%M:%S"));
            if let Some(ref priority) = note.note_priority {
                println!("优先级: {}", priority);
            }
            if let Some(ref tag) = note.note_tag {
                println!("标签: {}", tag);
            }
            println!("{}", "-".repeat(80));
        }
    }

    Ok(())
}

/// 添加笔记
pub fn add_note(database: &Database, form: &NoteForm) -> AnyResult<()> {
    let conn = database.get_connection();
    let id = note_dao::insert_note(conn, form)?;
    println!("✅ 笔记添加成功！ID: {}", id);
    Ok(())
}

/// 更新笔记
pub fn update_note(database: &Database, form: &NoteForm) -> AnyResult<()> {
    let conn = database.get_connection();
    note_dao::update_note(conn, form)?;
    println!("✅ 笔记更新成功");
    Ok(())
}

/// 删除笔记
pub fn delete_note(database: &Database, id: i32) -> AnyResult<()> {
    let conn = database.get_connection();
    note_dao::delete_note(conn, id)?;
    println!("✅ 笔记删除成功");
    Ok(())
}

/// 根据 ID 获取笔记
#[allow(dead_code)]
pub fn get_note_by_id(database: &Database, id: i32) -> AnyResult<Option<NoteForm>> {
    let conn = database.get_connection();
    note_dao::get_note_by_id(conn, id)
}

