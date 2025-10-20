# Scripts 文件夹说明

本文件夹包含TodoList项目的所有脚本文件。

## 运行脚本

### Windows
- `run.bat` - 开发环境启动脚本（自动选择debug/release版本）
- `start.bat` - 生产环境启动脚本

### Linux/macOS
- `run.sh` - 开发环境启动脚本（自动选择debug/release版本）
- `start.sh` - 生产环境启动脚本

## 提醒功能设置脚本

### Windows
- `setup_reminder_task.ps1` - Windows 任务计划程序设置脚本

#### 使用方法：
```powershell
# 以管理员身份运行 PowerShell
# 进入项目根目录
cd E:\no_game\git\todoList\todoList\project

# 先编译 release 版本
cargo build --release

# 运行设置脚本
powershell -ExecutionPolicy Bypass -File scripts\setup_reminder_task.ps1
```

#### 功能：
- 自动创建Windows任务计划
- 可选择提醒检查频率（每小时/30分钟/15分钟/自定义）
- 支持测试运行

## 注意事项

1. 所有脚本都会自动切换到项目根目录（scripts的上级目录）
2. 确保在使用提醒功能前已编译release版本
3. 提醒功能需要在应用的setting模式中配置开启

