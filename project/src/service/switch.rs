use crate::init::db_json;
use anyhow::Result as AnyResult;
use std::io::{self, Write};

/// 所有可用的模式
const AVAILABLE_MODES: &[&str] = &["memo", "review"];

/// 切换应用模式
pub fn switch_mode(json_config: &db_json::JsonConfig) -> AnyResult<()> {
    // 显示当前模式
    let current_mode = json_config.get("mode")?;
    println!("📌 当前模式: {}", current_mode);
    println!();
    
    // 显示可用模式
    println!("🔄 可切换的模式:");
    for (index, mode) in AVAILABLE_MODES.iter().enumerate() {
        println!("  {}. {}", index + 1, mode);
    }
    println!();
    
    // 循环读取用户输入，直到输入有效
    loop {
        print!("请输入要切换的模式名称（或输入 'cancel' 取消）: ");
        io::stdout().flush()?;
        
        // 读取用户输入
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        // 检查是否取消
        if input.eq_ignore_ascii_case("cancel") {
            println!("❌ 已取消切换模式");
            return Ok(());
        }
        
        // 检查输入是否为空
        if input.is_empty() {
            println!("⚠️  输入不能为空，请重新输入");
            println!();
            continue;
        }
        
        // 验证输入是否在可选范围内
        if !AVAILABLE_MODES.contains(&input) {
            println!("❌ 错误: 模式 '{}' 不存在", input);
            println!("   可用的模式有: {}", AVAILABLE_MODES.join(", "));
            println!("   请重新输入");
            println!();
            continue;
        }
        
        // 检查是否与当前模式相同
        if input == current_mode {
            println!("ℹ️  当前已是 '{}' 模式，无需切换", input);
            return Ok(());
        }
        
        // 执行切换
        json_config.set("mode", input)?;
        println!("✅ 成功切换到 '{}' 模式", input);
        println!("💡 提示: 输入 'help' 查看当前模式的可用命令");
        
        break;
    }
    
    Ok(())
}

/// 获取当前模式
#[allow(dead_code)]
pub fn get_current_mode(json_config: &db_json::JsonConfig) -> AnyResult<String> {
    json_config.get("mode")
}

/// 检查模式是否有效
#[allow(dead_code)]
pub fn is_valid_mode(mode: &str) -> bool {
    AVAILABLE_MODES.contains(&mode)
}

/// 获取所有可用模式
#[allow(dead_code)]
pub fn get_available_modes() -> &'static [&'static str] {
    AVAILABLE_MODES
}