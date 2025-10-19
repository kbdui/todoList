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
        _ => {
            println!("❌ 未知命令: '{}'", order);
            println!("💡 输入 'help' 查看可用命令");
        }
    }
    
    Ok(())
}