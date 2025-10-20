use anyhow::{Result as AnyResult, Context};
use std::fs;
use serde_json::Value;
use crate::init::db_json_content::JsonDataContent;

/// JSON数据管理器
pub struct JsonConfig {
    json_path: String,
}

impl JsonConfig {
    /// 创建或打开JSON文件
    /// json_path: JSON文件的完整路径
    /// 当前，若修改了config的结构，需删除config文件以便触发重新生成
    pub fn new(json_path: &str) -> AnyResult<Self> {
        // 如果文件不存在，创建默认的JSON文件
        if !std::path::Path::new(json_path).exists() {
            // 从 JsonDataContent 获取默认实例
            let default_data = JsonDataContent::default();
            
            // 序列化为JSON字符串
            let json_string = serde_json::to_string_pretty(&default_data)
                .context("Failed to serialize default JSON data")?;
            
            // 写入文件
            fs::write(json_path, json_string)
                .context("Failed to create default JSON file")?;
        }
        
        Ok(Self {
            json_path: json_path.to_string(),
        })
    }
    
    /// 读取整个JSON对象
    fn read_json(&self) -> AnyResult<Value> {
        // 检查文件是否存在
        if !std::path::Path::new(&self.json_path).exists() {
            return Err(anyhow::anyhow!(
                "JSON file does not exist: {}",
                self.json_path
            ));
        }
        
        // 读取文件内容
        let content = fs::read_to_string(&self.json_path)
            .context("Failed to read JSON file")?;
        
        // 解析JSON
        let data: Value = serde_json::from_str(&content)
            .context("Failed to parse JSON file")?;
        
        Ok(data)
    }
    
    /// 写入整个JSON对象
    fn write_json(&self, data: &Value) -> AnyResult<()> {
        // 将数据序列化为格式化的JSON字符串
        let json_string = serde_json::to_string_pretty(data)
            .context("Failed to serialize JSON data")?;
        
        // 写入文件
        fs::write(&self.json_path, json_string)
            .context("Failed to write JSON file")?;
        
        Ok(())
    }
    
    /// 获取指定字段的值（返回字符串）
    /// field_name: 要获取的字段名
    pub fn get(&self, field_name: &str) -> AnyResult<String> {
        let data = self.read_json()?;
        
        // 检查是否为对象
        let obj = data.as_object()
            .ok_or_else(|| anyhow::anyhow!("JSON root is not an object"))?;
        
        // 获取字段值
        let value = obj.get(field_name)
            .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", field_name))?;
        
        // 转换为字符串
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Null => Ok("null".to_string()),
            _ => Ok(value.to_string()),
        }
    }
    
    /// 设置指定字段的值
    /// field_name: 要设置的字段名
    /// value: 要设置的值（字符串）
    pub fn set(&self, field_name: &str, value: &str) -> AnyResult<()> {
        let mut data = self.read_json()?;
        
        // 检查是否为对象
        let obj = data.as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("JSON root is not an object"))?;
        
        // 设置字段值
        obj.insert(field_name.to_string(), Value::String(value.to_string()));
        
        // 写回文件
        self.write_json(&data)?;
        
        Ok(())
    }
    
    /// 获取指定字段的原始JSON值
    /// field_name: 要获取的字段名
    #[allow(dead_code)]
    pub fn get_value(&self, field_name: &str) -> AnyResult<Value> {
        let data = self.read_json()?;
        
        // 检查是否为对象
        let obj = data.as_object()
            .ok_or_else(|| anyhow::anyhow!("JSON root is not an object"))?;
        
        // 获取字段值
        let value = obj.get(field_name)
            .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", field_name))?;
        
        Ok(value.clone())
    }
    
    /// 设置指定字段的JSON值
    /// field_name: 要设置的字段名
    /// value: 要设置的JSON值
    #[allow(dead_code)]
    pub fn set_value(&self, field_name: &str, value: Value) -> AnyResult<()> {
        let mut data = self.read_json()?;
        
        // 检查是否为对象
        let obj = data.as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("JSON root is not an object"))?;
        
        // 设置字段值
        obj.insert(field_name.to_string(), value);
        
        // 写回文件
        self.write_json(&data)?;
        
        Ok(())
    }
}

