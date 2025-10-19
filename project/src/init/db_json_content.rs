use serde::{Deserialize, Serialize};

/// JSON数据结构（仅用于序列化/反序列化）
/// 在这里定义JSON文件中的所有字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonDataContent {
    pub mode: String,
    // 可以继续添加更多字段，例如：
    // pub version: String,
    // pub app_name: String,
    // pub max_users: i32,
    // pub debug: bool,
}

impl JsonDataContent {
    /// 创建默认的JSON数据实例
    /// 在这里设置所有字段的初始值
    /// 修改这里可以改变新建JSON文件的默认内容
    pub fn default() -> Self {
        Self {
            mode: "memo".to_string(),
            // 对应添加默认值，例如：
            // version: "1.0.0".to_string(),
            // app_name: "TodoList".to_string(),
            // max_users: 100,
            // debug: false,
        }
    }
}