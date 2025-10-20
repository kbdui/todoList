# 提醒功能快速开始指南

## 🚀 快速设置（3步）

### 步骤1：编译项目
```bash
cd project
cargo build --release
```

### 步骤2：设置Windows任务计划
```powershell
# 以管理员身份运行PowerShell
powershell -ExecutionPolicy Bypass -File scripts\setup_reminder_task.ps1

# 按照提示选择检查频率（推荐：每小时）
# 选择 "Y" 进行测试运行
```

### 步骤3：在应用中验证
```bash
# 运行应用
.\target\release\project.exe

# 或使用脚本
scripts\run.bat

# 切换到setting模式
> switch
输入: setting

# 查看提醒状态
> reminder-status

# 测试提醒功能
> test-reminder
```

## 💡 使用技巧

### 查看提醒历史
```
> switch
输入: setting

> reminder-history
```

### 临时关闭提醒
```
> switch
输入: setting

> reminder
选择: 2 (禁用提醒功能)
```

### 清理旧历史
```
> switch
输入: setting

> reminder-cleanup
输入天数: 30
确认: y
```

## 🎯 提醒时机

提醒功能会在以下情况触发：

1. **应用启动时** - 自动检查并显示
2. **Windows任务计划** - 定时后台检查（每小时/自定义）
3. **手动测试** - 使用 `test-reminder` 命令

## ⚙️ 修改配置

编辑 `database/config.json`：

```json
{
  "mode": "memo",
  "reminder": {
    "enabled": true,
    "notification_type": "console",  // "console" | "windows" | "both"
    "rules": [...]
  }
}
```

### notification_type 选项：
- `"console"` - 仅控制台通知
- `"windows"` - 仅Windows系统通知
- `"both"` - 两种都启用

## 🔍 故障排查

### 问题：收不到提醒
1. 检查提醒是否启用：`> reminder-status`
2. 检查任务计划是否运行：打开任务计划程序查看
3. 手动测试：`.\target\release\project.exe --check-reminders`

### 问题：重复提醒
- 提醒会自动记录历史，同一任务的同一类型提醒只会发送一次
- 如需重置，可清理提醒历史：`> reminder-cleanup`

### 问题：任务计划不执行
1. 确认任务计划已创建：`Get-ScheduledTask -TaskName "TodoListReminder"`
2. 手动运行测试：`Start-ScheduledTask -TaskName "TodoListReminder"`
3. 查看任务历史和错误日志

## 📊 命令速查

| 命令 | 功能 |
|------|------|
| `reminder` | 开关提醒功能 |
| `reminder-status` | 查看提醒状态 |
| `reminder-history` | 查看提醒历史 |
| `reminder-cleanup` | 清理旧历史 |
| `test-reminder` | 测试提醒功能 |

## 📝 日志位置

- 提醒日志：`database/reminder.log`
- 应用日志：`database/app.log`

---

**提示**：使用 `switch` 命令在不同模式间切换
- `memo` - 待办事项管理
- `review` - 查看统计
- `setting` - 系统设置（包含提醒功能）

