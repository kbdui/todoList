use crate::init::db_json;
use anyhow::Result as AnyResult;

// 打印帮助信息
pub fn print_help(json_config: &db_json::JsonConfig) -> AnyResult<()> {
    // 获取当前模式
    let mode = json_config.get("mode")?;
    println!("📌 当前应用模式: {}", mode);
    println!();

    println!("📋 可用命令列表:");
    println!("  help   - 显示此帮助信息");
    println!("  switch - 切换应用模式");
    println!("  exit   - 退出程序");
    println!();

    // 根据模式显示特定命令
    if mode == "memo" {
        print_memo_help();
    }

    Ok(())
}

// 打印memo可用指令
pub fn print_memo_help() {
    println!("📝 Memo 模式专用命令:");
    println!("  list   - 显示所有待办事项");
    println!("  insert - 添加待办事项");
    println!("  update - 更新待办事项");
    println!("  delete - 删除待办事项");
}