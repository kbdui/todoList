use anyhow::Result as AnyResult;
use crate::data::todo_list::TodoListForm;
use crate::dao::todo_list_dao;
use crate::init::database::Database;

// 输出所有的事项
pub fn show_all_todos(database: &Database) -> AnyResult<()> {
    let conn = database.get_connection();
    let todos = todo_list_dao::list_todos(conn)?;

    /// 打印所有任务到命令行（控制台）
    /// 检查 todos 是否为空，并根据结果输出相应的信息
    if todos.is_empty() {
        println!("暂无代办事项");
    } else {
        println!("All Todos:");
        for todo in &todos {
            println!("{:?}", todo); // 输出每个 Todo 项目
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
    Ok(())
}