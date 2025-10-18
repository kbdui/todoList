use std::fs;
use toml::Value;
use std::path::PathBuf;

// 获取配置文件的绝对路径
// 注意：程序必须从 project 目录启动，或确保当前工作目录下有 config.toml
fn get_config_path() -> PathBuf {
    PathBuf::from("config.toml")
        .canonicalize()
        .expect("Failed to find config.toml file. Make sure to run the program from the project directory.")
}

// 获取项目根目录（配置文件所在目录）
fn get_project_root() -> PathBuf {
    get_config_path()
        .parent()
        .expect("Failed to get project root directory")
        .to_path_buf()
}

// 加载配置信息
fn load_config() -> Value {
    let config_path = get_config_path();
    let config_content = fs::read_to_string(&config_path)
                .expect("Failed to read config file");

    let config: Value = toml::from_str(&config_content)
                .expect("Failed to parse config file");

    return config;
}

// 获取对应条目的信息，并返回数据库文件的完整路径
pub fn get_config_value(key1: &str, key2: Option<&str>) -> String {
    let config = load_config();
    
    // 获取项目根目录
    let project_root = get_project_root();
    
    // 拼接 database 文件夹路径
    let database_dir = project_root.join("database");
    
    // 从配置文件中获取值
    let config_value = match key2 {
        Some(k2) => config[key1][k2].as_str().expect("Failed to get config value"),
        None => config[key1].as_str().expect("Failed to get config value"),
    };
    
    // 拼接完整的数据库文件路径
    let db_path = database_dir.join(config_value);
    
    // 返回路径字符串
    db_path.to_str().expect("Failed to convert path to string").to_string()
}