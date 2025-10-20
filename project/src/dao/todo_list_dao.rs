
use anyhow::Result as AnyResult;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};

use crate::data::todo_list::TodoListForm;

// 类型转换
fn datetime_to_text(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn text_to_datetime(s: &str) -> AnyResult<DateTime<Utc>> {
    let fixed = DateTime::parse_from_rfc3339(s)?;
    Ok(fixed.with_timezone(&Utc))
}

// 将数据库行映射到TodoListForm
fn map_row(row: &Row) -> AnyResult<TodoListForm> {
    let id: i32 = row.get("id")?;
    let title: String = row.get("title")?;
    let description: Option<String> = row.get("description").ok();
    let completed_i: i32 = row.get("completed")?;
    let begin_time_s: String = row.get("begin_time")?;
    let end_time_s: Option<String> = row.get("end_time").ok();
    let key_message1: Option<String> = row.get("key_message1").ok();
    let key_message2: Option<String> = row.get("key_message2").ok();
    let key_message3: Option<String> = row.get("key_message3").ok();

    Ok(TodoListForm {
        id,
        title,
        description,
        completed: completed_i != 0,
        begin_time: text_to_datetime(&begin_time_s)?,
        end_time: match end_time_s {
            Some(s) => Some(text_to_datetime(&s)?),
            None => None,
        },
        key_message1,
        key_message2,
        key_message3,
    })
}

// 插入todo
pub fn insert_todo(conn: &Connection, form: &TodoListForm) -> AnyResult<i64> {
    let sql = r#"
        INSERT INTO todo_list (
            title, description, completed, begin_time, end_time,
            key_message1, key_message2, key_message3
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
    "#;

    let completed_i = if form.completed { 1 } else { 0 };
    conn.execute(
        sql,
        params![
            form.title,
            form.description,
            completed_i,
            datetime_to_text(&form.begin_time),
            form.end_time.as_ref().map(datetime_to_text),
            form.key_message1,
            form.key_message2,
            form.key_message3,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

// 根据id获取todo
pub fn get_todo_by_id(conn: &Connection, id: i32) -> AnyResult<Option<TodoListForm>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, title, description, completed, begin_time, end_time,
           key_message1, key_message2, key_message3 FROM todo_list WHERE id = ?1"#,
    )?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(map_row(row)?))
    } else {
        Ok(None)
    }
}

// 获取所有todo
pub fn list_todos(conn: &Connection) -> AnyResult<Vec<TodoListForm>> {
    let mut stmt = conn.prepare(
        r#"SELECT id, title, description, completed, begin_time, end_time,
           key_message1, key_message2, key_message3 FROM todo_list ORDER BY id DESC"#,
    )?;
    let mut rows = stmt.query([])?;
    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(map_row(row)?);
    }
    Ok(results)
}

// 更新todo
pub fn update_todo(conn: &Connection, form: &TodoListForm) -> AnyResult<usize> {
    let sql = r#"
        UPDATE todo_list SET
            title = ?1,
            description = ?2,
            completed = ?3,
            begin_time = ?4,
            end_time = ?5,
            key_message1 = ?6,
            key_message2 = ?7,
            key_message3 = ?8
        WHERE id = ?9
    "#;
    let completed_i = if form.completed { 1 } else { 0 };
    let rows = conn.execute(
        sql,
        params![
            form.title,
            form.description,
            completed_i,
            datetime_to_text(&form.begin_time),
            form.end_time.as_ref().map(datetime_to_text),
            form.key_message1,
            form.key_message2,
            form.key_message3,
            form.id,
        ],
    )?;
    Ok(rows as usize)
}

// 删除todo
pub fn delete_todo(conn: &Connection, id: i32) -> AnyResult<usize> {
    let rows = conn.execute("DELETE FROM todo_list WHERE id = ?1", params![id])?;
    Ok(rows as usize)
}

// 标记待办事项为完成
#[allow(dead_code)]
pub fn mark_as_completed(conn: &Connection, id: i32) -> AnyResult<usize> {
    let rows = conn.execute(
        "UPDATE todo_list SET completed = 1 WHERE id = ?1",
        params![id]
    )?;
    Ok(rows as usize)
}

// 标记待办事项为未完成
#[allow(dead_code)]
pub fn mark_as_pending(conn: &Connection, id: i32) -> AnyResult<usize> {
    let rows = conn.execute(
        "UPDATE todo_list SET completed = 0 WHERE id = ?1",
        params![id]
    )?;
    Ok(rows as usize)
}

// 切换待办事项的完成状态
pub fn toggle_completed(conn: &Connection, id: i32) -> AnyResult<usize> {
    let rows = conn.execute(
        "UPDATE todo_list SET completed = 1 - completed WHERE id = ?1",
        params![id]
    )?;
    Ok(rows as usize)
}

