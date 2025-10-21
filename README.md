# 📝 TodoList - 智能待办事项管理系统

> 基于 Rust 的高效命令行 Todo 应用，支持自动提醒、多模式管理、日志记录等功能

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg)](https://github.com)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## ✨ 核心功能

### 📋 待办事项管理
- ✅ **完整 CRUD 操作** - 创建、查看、更新、删除待办事项
- 📅 **时间管理** - 支持开始时间、截止时间设置
- 🏷️ **优先级设置** - 高、中、低三级优先级
- 📝 **备注功能** - 为每个任务添加详细备注
- ✔️ **状态跟踪** - 标记任务完成状态

### ⏰ 智能提醒系统
- 🔔 **自动提醒** - Windows 任务计划程序集成，后台自动检查
- ⚙️ **可配置规则** - 支持开始前 1 天、1 小时、逾期提醒
- 🕐 **自定义间隔** - 提醒检查频率可调（1-1440 分钟）
- 📊 **提醒历史** - 记录所有提醒历史，可查看和清理
- 🎯 **精准推送** - 避免重复提醒，智能去重

### 🎨 多模式操作
- **Memo 模式** - 专注于创建和编辑待办事项
- **Review 模式** - 查看和检索待办事项
- **Reminder 模式** - 管理提醒功能和查看提醒历史
- **一键切换** - 使用 `switch` 命令快速切换模式

### 📊 高级功能
- 📂 **日志记录** - 应用日志和提醒日志分别记录
- 💾 **SQLite 数据库** - 可靠的本地数据存储
- 🔍 **灵活查询** - 支持按状态、优先级筛选
- 🖥️ **跨平台** - Windows、Linux、macOS 全平台支持

---

## 🚀 快速开始

### 📦 环境要求

- **Rust** 1.70+ (开发)
- **操作系统**: Windows 10+, Linux, macOS
- **PowerShell** 5.0+ (Windows 提醒功能)

### ⚡ 快速运行

#### Windows 用户
```bash
# 1. 克隆项目
git clone <repository-url>
cd todoList/project

# 2. 编译项目
cargo build --release

# 3. 启动程序
scripts\run.bat
```

#### Linux/macOS 用户
```bash
# 1. 克隆项目
git clone <repository-url>
cd todoList/project

# 2. 编译项目
cargo build --release

# 3. 添加执行权限
chmod +x scripts/*.sh

# 4. 启动程序
scripts/run.sh
```

### 🎯 首次使用

启动后，程序会自动：
1. ✅ 创建 SQLite 数据库
2. ✅ 初始化配置文件
3. ✅ 进入交互式命令行界面

输入 `help` 查看所有可用命令！

---

## 📁 项目结构

```
todoList/
├── project/                    # 主项目目录
│   ├── src/                    # 源代码
│   │   ├── main.rs            # 程序入口
│   │   ├── cli/               # 命令行界面模块
│   │   │   ├── mod.rs
│   │   │   ├── help_distribute.rs  # 命令分发器
│   │   │   ├── todo_list_cli.rs    # 待办事项命令
│   │   │   ├── review_cli.rs       # 查看命令
│   │   │   └── reminder_cli.rs     # 提醒命令
│   │   ├── dao/               # 数据访问层
│   │   │   ├── mod.rs
│   │   │   ├── todo_list_dao.rs
│   │   │   ├── note_dao.rs
│   │   │   └── reminder_dao.rs
│   │   ├── data/              # 数据模型
│   │   │   ├── mod.rs
│   │   │   ├── todo_list.rs
│   │   │   ├── note.rs
│   │   │   └── reminder.rs
│   │   ├── service/           # 业务逻辑层
│   │   │   ├── mod.rs
│   │   │   ├── todo_list_serv.rs
│   │   │   ├── reminder_serv.rs
│   │   │   ├── notifier.rs
│   │   │   ├── logger.rs
│   │   │   └── help.rs
│   │   ├── init/              # 初始化模块
│   │   │   ├── mod.rs
│   │   │   ├── database.rs
│   │   │   ├── db_json.rs
│   │   │   └── config_load.rs
│   │   └── runner/            # 运行模式
│   │       ├── mod.rs
│   │       └── reminder.rs    # 提醒检查模式
│   ├── scripts/               # 脚本文件
│   │   ├── run.bat           # Windows 启动脚本（开发）
│   │   ├── start.bat         # Windows 启动脚本（生产）
│   │   ├── run.sh            # Linux/macOS 启动脚本（开发）
│   │   ├── start.sh          # Linux/macOS 启动脚本（生产）
│   │   ├── create_task.ps1   # 自动创建提醒任务
│   │   └── setup_reminder_task.ps1  # 手动配置提醒任务
│   ├── database/             # 数据存储
│   │   ├── todo.db          # SQLite 数据库
│   │   ├── config.json      # 配置文件
│   │   ├── app.log          # 应用日志
│   │   └── reminder.log     # 提醒日志
│   ├── document/             # 文档
│   ├── Cargo.toml           # Rust 项目配置
│   └── config.toml          # 应用配置
└── README.md                # 本文件
```

---

## 📖 使用指南

### 🎮 基础命令

#### 全局命令（所有模式通用）
```bash
help          # 显示帮助信息
switch        # 切换操作模式 (memo/review/reminder)
exit          # 退出程序
```

### 📝 Memo 模式（待办事项管理）

```bash
# 切换到 Memo 模式
> switch
选择模式: 1  # 或直接输入 memo

# 创建待办事项
> add
标题: 完成项目报告
开始时间: 2025-10-25 09:00:00
截止时间: 2025-10-25 18:00:00
优先级 (1-高/2-中/3-低): 1

# 添加备注
> add-note
任务ID: 1
备注内容: 需要包含数据分析和图表

# 标记完成
> complete
任务ID: 1

# 删除任务
> delete
任务ID: 2
```

### 🔍 Review 模式（查看和检索）

```bash
# 切换到 Review 模式
> switch
选择模式: 2  # 或直接输入 review

# 查看所有待办
> list

# 查看未完成任务
> list-uncompleted

# 查看已完成任务
> list-completed

# 查看高优先级任务
> list-high

# 查看任务详情
> detail
任务ID: 1
```

### ⏰ Reminder 模式（提醒管理）

```bash
# 切换到 Reminder 模式
> switch
选择模式: 3  # 或直接输入 reminder

# 启用/禁用提醒功能
> reminder
选择: 1  # 启用
检查间隔（分钟）: 15  # 每15分钟检查一次（默认值）

# 设置通知类型（重要！）
> reminder-type
选择: 2  # Windows 通知（定时任务建议使用，避免弹窗）

# 查看提醒状态
> reminder-status

# 查看提醒历史
> reminder-history

# 清理旧提醒记录
> reminder-cleanup

# 测试提醒功能
> test-reminder
```

---

## ⚙️ 提醒功能详解

### 🔧 配置说明

提醒配置位于 `database/config.json`：

```json
{
  "mode": "reminder",
  "reminder": {
    "enabled": true,
    "check_interval_minutes": 15,
    "notification_type": "both",
    "rules": [
      {
        "message_template": "📅 任务「{title}」(ID:{id}) 将在1天后开始",
        "rule_type": "before_start",
        "seconds_before": 86400
      },
      {
        "message_template": "⏰ 任务「{title}」(ID:{id}) 将在1小时后开始！",
        "rule_type": "before_start",
        "seconds_before": 3600
      },
      {
        "message_template": "❌ 任务「{title}」(ID:{id}) 已超过开始时间！",
        "rule_type": "overdue",
        "seconds_before": null
      }
    ]
  }
}
```

### 📋 提醒规则类型

| 规则类型 | 说明 | 触发时机 |
|---------|------|---------|
| `before_start` | 开始前提醒 | 距离任务开始时间前 N 秒 |
| `overdue` | 逾期提醒 | 已超过开始时间但未完成 |

### 🔔 通知类型设置

| 类型 | 说明 | 适用场景 |
|------|------|---------|
| **📟 控制台通知** | 仅在终端显示 | 开发调试、手动运行 |
| **🪟 Windows 通知** | 系统托盘弹窗 | **后台定时任务（推荐）** |
| **🔔 双重通知** | 同时使用两种 | 需要多重提醒 |

**💡 重要提示：**
- 默认使用 **双重通知**（控制台 + Windows），可根据需要修改
- 使用定时任务时，建议仅使用 **Windows 通知**，避免弹出控制台窗口
- 使用 `reminder-type` 命令随时更改通知类型
- 默认检查间隔为 **15 分钟**，适合大多数使用场景

### 🚀 自动化设置（Windows）

#### 方式一：自动设置（推荐）
1. 在程序中启用提醒功能（`reminder` 命令）
2. 设置检查间隔（默认 15 分钟，可自定义 1-1440 分钟）
3. （可选）设置通知类型（默认为双重通知，可改为仅 Windows 通知）
4. 重启程序 - `run.bat` 会自动创建 Windows 任务计划（已配置为隐藏运行）
5. 如需修改间隔，使用 `reminder` 命令选择"修改检查间隔"，重启后自动生效

#### 方式二：手动配置
运行交互式配置工具：
```powershell
powershell -ExecutionPolicy Bypass -File scripts\setup_reminder_task.ps1
```

### 📊 提醒历史管理

- **自动记录** - 所有提醒自动记录到数据库
- **防止重复** - 同一任务的同类型提醒只发送一次
- **历史查看** - `reminder-history` 命令查看
- **定期清理** - `reminder-cleanup` 命令清理旧记录

---

## 🎯 部署指南

### 📦 编译发布版本

```bash
cd project
cargo build --release
```

生成的可执行文件：
- **Windows**: `target/release/project.exe`
- **Linux/macOS**: `target/release/project`

### 📂 部署文件清单

```
部署目录/
├── project.exe (或 project)    # 可执行文件
├── config.toml                 # 配置文件
├── scripts/
│   ├── start.bat              # Windows 启动脚本
│   └── start.sh               # Linux/macOS 启动脚本
└── database/                   # 数据目录（自动创建）
```

### 🚀 用户使用

**Windows:**
```bash
双击 start.bat
```

**Linux/macOS:**
```bash
./start.sh
```

详细部署说明请参考：[project/document/部署说明.md](project/document/部署说明.md)

---

## 🛠️ 开发指南

### 🔨 环境搭建

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 克隆项目
git clone <repository-url>
cd todoList/project

# 3. 安装依赖
cargo build

# 4. 运行开发版本
cargo run
```

### 📦 依赖项

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }  # 时间处理
rusqlite = { version = "0.37.0", features = ["bundled"] }  # SQLite
serde = { version = "1.0", features = ["derive"] }  # 序列化
serde_json = "1.0"  # JSON 处理
anyhow = "1.0"  # 错误处理
toml = "0.9.8"  # 配置文件

[target.'cfg(windows)'.dependencies]
winrt-notification = "0.5"  # Windows 通知
```

### 🧪 构建脚本

```bash
# Windows 快速构建
project\build-release.bat

# Linux/macOS 快速构建
project/build-release.sh
```

### 📝 代码规范

- 使用 `mod.rs` 模式组织模块
- 错误处理使用 `anyhow::Result`
- 数据访问层使用 DAO 模式
- 业务逻辑与 UI 分离

---

## ❓ 常见问题

### Q: 提示 "Failed to find config.toml"
**A:** 没有使用启动脚本。请使用 `run.bat` 或 `start.bat` 启动。

### Q: 提醒功能不工作
**A:** 
1. 检查是否已在程序中启用提醒功能
2. Windows 用户：打开任务计划程序检查 `TodoListReminder` 任务是否存在
3. 查看 `database/reminder.log` 日志文件

### Q: 启动脚本窗口一闪而过
**A:** 在终端/命令行中手动运行脚本查看详细错误信息。

### Q: 如何修改提醒检查频率
**A:** 
- **推荐方式**: 在程序中使用 `reminder` 命令 → 选择"修改检查间隔"→ 重启程序
- 方式 2: 直接编辑 `database/config.json` 中的 `check_interval_minutes` → 重启程序
- 方式 3: 运行 `setup_reminder_task.ps1` 重新配置
- **注意**: 修改后必须重启程序，`run.bat` 会自动检测变化并更新任务计划

### Q: 项目移动位置后提醒失效
**A:** 运行一次 `run.bat`，脚本会自动检测并更新任务路径。

### Q: 定时任务每次检查都弹出控制台窗口
**A:** 
1. 使用 `reminder-type` 命令将通知类型改为 **Windows 通知**
2. 重启程序，`run.bat` 会自动更新任务配置为隐藏运行
3. 如果仍有问题，手动删除任务后重新创建

---

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

---

## 🙏 致谢

- [Rust](https://www.rust-lang.org/) - 强大的系统编程语言
- [rusqlite](https://github.com/rusqlite/rusqlite) - Rust SQLite 绑定
- [chrono](https://github.com/chronotope/chrono) - Rust 时间处理库

---

## 📮 联系方式

如有问题或建议，欢迎提交 Issue 或 Pull Request。

---

<p align="center">
  <i>用心管理每一个待办事项 ✨</i>
</p>
