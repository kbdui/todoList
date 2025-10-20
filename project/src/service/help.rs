
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
    match mode.as_str() {
        "memo" => print_memo_help(),
        "review" => print_review_help(),
        _ => {
            println!("⚠️  未知模式: {}", mode);
        }
    }

    Ok(())
}

// 打印 memo 模式可用指令
pub fn print_memo_help() {
    println!("📝 Memo 模式专用命令:");
    println!("  list   - 显示所有待办事项");
    println!("  new    - 创建新的待办事项");
    println!("  insert - 添加待办事项");
    println!("  update - 更新待办事项");
    println!("  delete - 删除待办事项");
    println!("  toggle - 切换待办事项完成状态");
    println!("  note   - 管理待办事项的笔记");
    println!();
    println!("💡 提示: 使用 'switch' 切换到 'review' 模式查看完成统计");
}

// 打印 review 模式可用指令
pub fn print_review_help() {
    println!("📊 Review 模式专用命令:");
    println!("  stats     - 显示整体统计信息");
    println!("  completed - 显示已完成的待办事项");
    println!("  pending   - 显示未完成的待办事项");
    println!("  notes     - 显示最近的笔记（最多10条）");
}