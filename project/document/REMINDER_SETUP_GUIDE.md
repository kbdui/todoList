# 提醒功能设置指南

## 🎯 设计理念

提醒功能采用**智能自动设置**的方式，用户只需在应用中启用提醒功能，程序会自动完成定时任务的设置。

## 🚀 快速开始（2步）

### 步骤1：启动应用并启用提醒功能

```bash
# 运行应用
scripts\run.bat

# 切换到setting模式
> switch
输入: setting

# 启用提醒功能
> reminder
选择: 1 (启用提醒功能)

# 是否现在重启程序？(Y/N): Y
```

### 步骤2：自动完成

当您选择重启程序后：
- ✅ 程序自动退出
- ✅ 再次运行 `scripts\run.bat`
- ✅ 脚本检测到提醒功能已启用但未设置定时任务
- ✅ **自动设置定时任务（每小时检查一次）**
- ✅ 程序正常启动

就这么简单！🎉

## 📋 工作流程

```
┌─────────────────────────────────────────┐
│  用户在应用中启用提醒功能                 │
│  (setting模式 > reminder > 选择1)         │
└───────────────┬─────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│  提示是否重启程序？                       │
│  (选择 Y)                                │
└───────────────┬─────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│  程序退出                                 │
└───────────────┬─────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│  用户再次运行 scripts\run.bat             │
└───────────────┬─────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│  启动脚本自动检测：                       │
│  1. 提醒功能是否启用？(检查config.json)   │
│  2. 定时任务是否已设置？                  │
└───────────────┬─────────────────────────┘
                │
                ▼
        ┌───────┴───────┐
        │ 已启用 && 未设置 │
        │               │
        ▼               ▼
┌─────────────┐   ┌─────────────┐
│  自动设置    │   │  直接启动    │
│  定时任务    │   │  程序        │
└──────┬──────┘   └─────────────┘
       │
       ▼
┌─────────────────────────────────────────┐
│  创建Windows任务计划：                    │
│  • 任务名：TodoListReminder               │
│  • 频率：每小时执行一次                   │
│  • 命令：project.exe --check-reminders    │
│  • 权限：当前用户                         │
└───────────────┬─────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────┐
│  ✅ 定时任务设置完成                      │
│  ✅ 程序正常启动                          │
└─────────────────────────────────────────┘
```

## ⚙️ 配置说明

### 默认配置

启动脚本会使用以下默认参数自动设置定时任务：

- **检查频率**：每小时（60分钟）
- **执行命令**：`project.exe --check-reminders`
- **权限级别**：当前用户（Limited）
- **电池策略**：允许使用电池时运行
- **超时时间**：5分钟
- **重试策略**：失败后重试3次，间隔1分钟

### 自定义配置

如果需要修改检查频率或其他参数，可以：

1. **方法1：删除任务后重新设置**
   ```powershell
   # 删除现有任务
   Unregister-ScheduledTask -TaskName "TodoListReminder" -Confirm:$false
   
   # 再次运行程序，会自动重新创建（使用默认配置）
   scripts\run.bat
   ```

2. **方法2：手动修改任务计划**
   ```
   Win + R → 输入 taskschd.msc → 找到 TodoListReminder → 右键属性 → 修改触发器
   ```

3. **方法3：使用高级设置脚本**
   ```powershell
   # 如果需要自定义频率等参数
   powershell -ExecutionPolicy Bypass -File scripts\setup_reminder_task.ps1
   ```

## 💡 使用技巧

### 查看提醒状态

```bash
> switch
输入: setting

> reminder-status
```

### 测试提醒功能

```bash
> switch
输入: setting

> test-reminder
```

### 禁用提醒功能

```bash
> switch
输入: setting

> reminder
选择: 2 (禁用提醒功能)
```

**注意**：禁用提醒功能不会删除定时任务，只是暂停提醒检查。

### 查看提醒历史

```bash
> switch
输入: setting

> reminder-history
```

### 清理旧历史

```bash
> switch
输入: setting

> reminder-cleanup
输入天数: 30  # 保留最近30天的记录
```

## 🔍 故障排查

### 问题1：定时任务未自动创建

**可能原因**：
- 没有编译项目（找不到可执行文件）
- 权限不足

**解决方法**：
```bash
# 1. 先编译项目
cargo build --release

# 2. 再次运行
scripts\run.bat
```

### 问题2：提醒功能不工作

**检查步骤**：
1. 确认提醒功能已启用
   ```bash
   > reminder-status
   ```

2. 确认定时任务已创建
   ```powershell
   Get-ScheduledTask -TaskName "TodoListReminder"
   ```

3. 手动测试
   ```bash
   .\target\release\project.exe --check-reminders
   ```

### 问题3：收不到通知

**解决方法**：
1. 检查通知类型配置（`database/config.json`）
2. 测试Windows通知权限
3. 查看提醒日志（`database/reminder.log`）

## 📊 配置文件说明

`database/config.json` 示例：

```json
{
  "mode": "memo",
  "reminder": {
    "enabled": true,              // 提醒功能开关
    "notification_type": "console", // 通知类型: console/windows/both
    "rules": [
      {
        "rule_type": "before_deadline",
        "seconds_before": 86400,  // 提前24小时
        "message_template": "📅 任务「{title}」(ID:{id}) 将在1天后到期"
      },
      {
        "rule_type": "before_deadline",
        "seconds_before": 3600,   // 提前1小时
        "message_template": "⏰ 任务「{title}」(ID:{id}) 将在1小时后到期！"
      },
      {
        "rule_type": "overdue",
        "seconds_before": null,   // 逾期提醒
        "message_template": "❌ 任务「{title}」(ID:{id}) 已逾期！"
      }
    ]
  }
}
```

## 🎯 总结

- **✅ 无需手动操作**：在应用中启用提醒功能即可
- **✅ 自动化设置**：重启程序时自动创建定时任务
- **✅ 开箱即用**：默认配置适合大多数场景
- **✅ 灵活可控**：可以随时启用/禁用/查看状态

只需两步，轻松享受自动提醒功能！🎉

