/// 配置重置模块
/// 负责在程序启动时重置配置标记
use crate::init::db_json::JsonConfig;
use crate::init::db_json_content::ReminderConfig;
use anyhow::Result as AnyResult;

/// 重置提醒配置的 is_changed 标记
/// 
/// 在程序启动时调用，将 is_changed 设置为 true
/// 表示配置已同步到任务计划程序
pub fn reset_reminder_changed_flag(json_config: &JsonConfig) -> AnyResult<()> {
    // 读取当前配置
    let mut config = match json_config.get_value("reminder") {
        Ok(value) => {
            serde_json::from_value::<ReminderConfig>(value)?
        }
        Err(_) => ReminderConfig::default(),
    };
    
    // 如果 is_changed 已经是 true，无需更新
    if config.is_changed {
        return Ok(());
    }
    
    // 重置 is_changed 标记
    config.is_changed = true;
    let value = serde_json::to_value(config)?;
    json_config.set_value("reminder", value)?;
    
    Ok(())
}

/// 标记提醒配置已改变
/// 
/// 当用户修改检查间隔等配置时调用
/// 将 is_changed 设置为 false，提示需要更新任务计划程序
pub fn mark_reminder_config_changed(json_config: &JsonConfig) -> AnyResult<()> {
    let mut config = match json_config.get_value("reminder") {
        Ok(value) => {
            serde_json::from_value::<ReminderConfig>(value)?
        }
        Err(_) => ReminderConfig::default(),
    };
    
    config.is_changed = false;
    let value = serde_json::to_value(config)?;
    json_config.set_value("reminder", value)?;
    
    Ok(())
}

