use crate::data::reminder::{ReminderHistory, ReminderType};
use rusqlite::{Connection, Result};
use chrono::Utc;

/// 检查是否已经发送过提醒
pub fn has_been_notified(
    conn: &Connection,
    todo_id: i32,
    reminder_type: &ReminderType,
) -> Result<bool> {
    let sql = r#"
        SELECT COUNT(*) 
        FROM reminder_history 
        WHERE todo_id = ?1 AND reminder_type = ?2
    "#;
    
    let count: i32 = conn.query_row(
        sql,
        rusqlite::params![todo_id, reminder_type.to_string()],
        |row| row.get(0),
    )?;
    
    Ok(count > 0)
}

/// 记录提醒历史
pub fn record_notification(
    conn: &Connection,
    todo_id: i32,
    reminder_type: &ReminderType,
) -> Result<()> {
    let sql = r#"
        INSERT INTO reminder_history (todo_id, reminder_time, reminder_type, notified)
        VALUES (?1, ?2, ?3, 1)
    "#;
    
    conn.execute(
        sql,
        rusqlite::params![
            todo_id,
            Utc::now().to_rfc3339(),
            reminder_type.to_string(),
        ],
    )?;
    
    Ok(())
}

/// 获取所有提醒历史
pub fn get_all_reminders(conn: &Connection) -> Result<Vec<ReminderHistory>> {
    let sql = r#"
        SELECT id, todo_id, reminder_time, reminder_type, notified
        FROM reminder_history
        ORDER BY reminder_time DESC
        LIMIT 100
    "#;
    
    let mut stmt = conn.prepare(sql)?;
    let reminder_iter = stmt.query_map([], |row| {
        Ok(ReminderHistory {
            id: row.get(0)?,
            todo_id: row.get(1)?,
            reminder_time: row.get::<_, String>(2)?
                .parse()
                .unwrap_or_else(|_| Utc::now()),
            reminder_type: row.get(3)?,
            notified: row.get::<_, i32>(4)? == 1,
        })
    })?;
    
    let mut reminders = Vec::new();
    for reminder in reminder_iter {
        reminders.push(reminder?);
    }
    
    Ok(reminders)
}

/// 清理旧的提醒历史
pub fn cleanup_old_history(conn: &Connection, days: i64) -> Result<usize> {
    use chrono::Duration;
    let cutoff_date = Utc::now() - Duration::days(days);
    
    let sql = "DELETE FROM reminder_history WHERE reminder_time <= ?1";
    let deleted = conn.execute(sql, [cutoff_date.to_rfc3339()])?;
    
    Ok(deleted)
}

