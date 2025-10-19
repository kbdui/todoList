use anyhow::Result as AnyResult;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};

use crate::data::note::NoteForm;

// DateTime 和文本互转
fn datetime_to_text(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn text_to_datetime(s: &str) -> AnyResult<DateTime<Utc>> {
    let fixed = DateTime::parse_from_rfc3339(s)?;
    Ok(fixed.with_timezone(&Utc))
}

// 将数据库行映射到 NoteForm
fn map_row(row: &Row) -> AnyResult<NoteForm> {
    let id: i32 = row.get("id")?;
    let todo_id: i32 = row.get("todo_id")?;
    let note_title: String = row.get("note_title")?;
    let note_content: String = row.get("note_content")?;
    let note_time_s: String = row.get("note_time")?;
    let noter: Option<String> = row.get("noter").ok();
    let note_type: Option<String> = row.get("note_type").ok();
    let note_status: Option<String> = row.get("note_status").ok();
    let note_tag: Option<String> = row.get("note_tag").ok();
    let note_priority: Option<String> = row.get("note_priority").ok();

    Ok(NoteForm {
        id,
        todo_id,
        note_title,
        note_content,
        note_time: text_to_datetime(&note_time_s)?,
        noter,
        note_type,
        note_status,
        note_tag,
        note_priority,
    })
}

// 插入笔记
pub fn insert_note(conn: &Connection, form: &NoteForm) -> AnyResult<i64> {
    let sql = r#"
        INSERT INTO notes (
            todo_id, note_title, note_content, note_time, noter,
            note_type, note_status, note_tag, note_priority
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
    "#;

    conn.execute(
        sql,
        params![
            form.todo_id,
            form.note_title,
            form.note_content,
            datetime_to_text(&form.note_time),
            form.noter,
            form.note_type,
            form.note_status,
            form.note_tag,
            form.note_priority,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

// 根据 ID 获取笔记
pub fn get_note_by_id(conn: &Connection, id: i32) -> AnyResult<Option<NoteForm>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, todo_id, note_title, note_content, note_time, noter,
           note_type, note_status, note_tag, note_priority FROM notes WHERE id = ?1"#,
    )?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(map_row(row)?))
    } else {
        Ok(None)
    }
}

// 获取某个 todo 项目的所有笔记
pub fn list_notes_by_todo_id(conn: &Connection, todo_id: i32) -> AnyResult<Vec<NoteForm>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, todo_id, note_title, note_content, note_time, noter,
           note_type, note_status, note_tag, note_priority FROM notes 
           WHERE todo_id = ?1 ORDER BY note_time DESC"#,
    )?;
    let mut rows = stmt.query(params![todo_id])?;
    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(map_row(row)?);
    }
    Ok(results)
}

// 更新笔记
pub fn update_note(conn: &Connection, form: &NoteForm) -> AnyResult<usize> {
    let sql = r#"
        UPDATE notes SET
            note_title = ?1,
            note_content = ?2,
            note_time = ?3,
            noter = ?4,
            note_type = ?5,
            note_status = ?6,
            note_tag = ?7,
            note_priority = ?8
        WHERE id = ?9
    "#;
    let rows = conn.execute(
        sql,
        params![
            form.note_title,
            form.note_content,
            datetime_to_text(&form.note_time),
            form.noter,
            form.note_type,
            form.note_status,
            form.note_tag,
            form.note_priority,
            form.id,
        ],
    )?;
    Ok(rows as usize)
}

// 删除笔记
pub fn delete_note(conn: &Connection, id: i32) -> AnyResult<usize> {
    let rows = conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(rows as usize)
}

// 删除某个 todo 项目的所有笔记
#[allow(dead_code)]
pub fn delete_notes_by_todo_id(conn: &Connection, todo_id: i32) -> AnyResult<usize> {
    let rows = conn.execute("DELETE FROM notes WHERE todo_id = ?1", params![todo_id])?;
    Ok(rows as usize)
}

