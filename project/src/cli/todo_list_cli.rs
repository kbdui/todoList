use crate::service::todo_list_serv;
use crate::init::database;
use crate::init::config_load;
use anyhow::Result as AnyResult;
use crate::service::help;

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
        "new" => {
            todo_list_serv::create_new_todo(&db)?;
        }
        "help" => {
            help::print_help();
        }
        "insert" => {
            todo_list_serv::add_todo(&db, &form)?;
        }
        "delete" => {
            todo_list_serv::delete_todo(&db, id)?;
        }
        "update" => {
            todo_list_serv::update_todo(&db, &form)?;
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