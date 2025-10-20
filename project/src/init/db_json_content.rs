use serde::{Deserialize, Serialize};

/// JSON数据结构（仅用于序列化/反序列化）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonDataContent {
    pub mode: String,
    #[serde(default)]
    pub reminder: ReminderConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReminderConfig {
    pub enabled: bool,
    pub notification_type: String,
    pub rules: Vec<ReminderRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReminderRule {
    pub rule_type: String,
    pub seconds_before: Option<i64>,
    pub message_template: String,
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            notification_type: "console".to_string(),
            rules: vec![
                ReminderRule {
                    rule_type: "before_deadline".to_string(),
                    seconds_before: Some(86400),
                    message_template: "📅 任务「{title}」(ID:{id}) 将在1天后到期".to_string(),
                },
                ReminderRule {
                    rule_type: "before_deadline".to_string(),
                    seconds_before: Some(3600),
                    message_template: "⏰ 任务「{title}」(ID:{id}) 将在1小时后到期！".to_string(),
                },
                ReminderRule {
                    rule_type: "overdue".to_string(),
                    seconds_before: None,
                    message_template: "❌ 任务「{title}」(ID:{id}) 已逾期！".to_string(),
                },
            ],
        }
    }
}

impl JsonDataContent {
    /// 创建默认的JSON数据实例
    pub fn default() -> Self {
        Self {
            mode: "memo".to_string(),
            reminder: ReminderConfig::default(),
        }
    }
}