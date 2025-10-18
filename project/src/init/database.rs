use rusqlite::{Connection};
use anyhow::{Result as AnyResult};

pub struct Database {
    conn: Connection,
}

impl Database {
    /// 创建或打开数据库文件
    /// Self的意思是返回类型自身
    pub fn new(db_path: &str) -> AnyResult<Self> {
        let conn = Connection::open(db_path)?;

        Ok(Self { conn })
    }

    // 初始化数据库表结构
    pub fn initialize_tables(&self) -> AnyResult<()> {
        self.create_todo_list_table()?;
        Ok(())
    }

    // 获取数据库连接
    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    // 创建todo_list表
    pub fn create_todo_list_table(&self) -> AnyResult<()> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS todo_list (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            completed INTEGER NOT NULL DEFAULT 0,
            begin_time TEXT NOT NULL,
            end_time TEXT,
            key_message1 TEXT,
            key_message2 TEXT,
            key_message3 TEXT
        )
        "#;
        self.conn.execute(sql, [])?;
        Ok(())
    }
}