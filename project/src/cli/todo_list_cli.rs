use crate::service::todo_list_serv;
use crate::init::database;

// 暂时先这么写，后面再改
pub fn order_check(order: &str) {
    let db = database::Database;

    db.initialize_tables();

    match order {
        "list" => {
            todo_list_serv::show_all_todos(&db);
        }
        _ => {
            println!("命令无效");
        }
    }
}