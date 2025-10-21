# 自动创建提醒任务的脚本
# 由 run.bat 调用

$ErrorActionPreference = "Stop"

try {
    # 获取项目路径
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $projectPath = Split-Path -Parent $scriptDir
    
    # 读取配置文件（UTF8编码）
    $configPath = Join-Path $projectPath "database\config.json"
    $json = Get-Content $configPath -Raw -Encoding UTF8 | ConvertFrom-Json
    
    # 获取检查间隔（默认60分钟）
    $intervalMinutes = if ($json.reminder.check_interval_minutes) { 
        $json.reminder.check_interval_minutes 
    } else { 
        60 
    }
    
    # 确定可执行文件路径
    $exePath = Join-Path $projectPath "target\release\project.exe"
    if (-Not (Test-Path $exePath)) {
        $exePath = Join-Path $projectPath "target\debug\project.exe"
    }
    
    if (-Not (Test-Path $exePath)) {
        Write-Host "找不到可执行文件" -ForegroundColor Red
        exit 1
    }
    
    # 创建任务动作
    $action = New-ScheduledTaskAction `
        -Execute $exePath `
        -Argument "--check-reminders" `
        -WorkingDirectory $projectPath
    
    # 创建触发器
    $trigger = New-ScheduledTaskTrigger `
        -Once `
        -At (Get-Date) `
        -RepetitionInterval (New-TimeSpan -Minutes $intervalMinutes)
    
    # 创建任务设置
    $settings = New-ScheduledTaskSettingsSet `
        -AllowStartIfOnBatteries `
        -DontStopIfGoingOnBatteries `
        -StartWhenAvailable `
        -RunOnlyIfNetworkAvailable:$false `
        -ExecutionTimeLimit (New-TimeSpan -Minutes 5) `
        -RestartCount 3 `
        -RestartInterval (New-TimeSpan -Minutes 1)
    
    # 创建任务主体
    $principal = New-ScheduledTaskPrincipal `
        -UserId $env:USERNAME `
        -LogonType Interactive `
        -RunLevel Limited
    
    # 先尝试删除已存在的任务（如果有）
    Unregister-ScheduledTask -TaskName "TodoListReminder" -Confirm:$false -ErrorAction SilentlyContinue | Out-Null
    
    # 注册任务
    $description = "TodoList reminder task - Check every $intervalMinutes minutes"
    Register-ScheduledTask `
        -TaskName "TodoListReminder" `
        -Action $action `
        -Trigger $trigger `
        -Settings $settings `
        -Principal $principal `
        -Description $description `
        -ErrorAction Stop | Out-Null
    
    Write-Host "Task created: check every $intervalMinutes minutes" -ForegroundColor Green
    exit 0
    
} catch {
    Write-Host "Failed to create task: $_" -ForegroundColor Red
    exit 1
}

