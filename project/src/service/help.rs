
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
        "reminder" => print_reminder_help(),
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
    println!("  insert - 添加待办事项（同上）");
    println!("  update - 更新待办事项");
    println!("  delete - 删除待办事项");
    println!("  toggle - 切换待办事项完成状态");
    println!("  note   - 管理待办事项的笔记");
    println!();
    println!("💡 提示: 使用 'switch' 切换到其他模式 (review/reminder)");
}

// 打印 review 模式可用指令
pub fn print_review_help() {
    println!("📊 Review 模式专用命令:");
    println!("  stats     - 显示整体统计信息");
    println!("  completed - 显示已完成的待办事项");
    println!("  pending   - 显示未完成的待办事项");
    println!("  notes     - 显示最近的笔记（最多10条）");
}

// 打印 reminder 模式可用指令
pub fn print_reminder_help() {
    println!("⏰ Reminder 模式专用命令:");
    println!("  reminder         - 提醒功能开关设置（启用后重启程序自动设置定时任务）");
    println!("  reminder-status  - 查看提醒功能状态");
    println!("  reminder-history - 查看提醒历史记录");
    println!("  reminder-cleanup - 清理旧提醒历史");
    println!("  test-reminder    - 测试提醒功能");
    println!();
    println!("💡 提示: 启用提醒功能后，程序会在重启时自动设置定时任务（每小时检查一次）");
}