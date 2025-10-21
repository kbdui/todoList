# setup_reminder_task.ps1
# TodoList 提醒功能 - Windows 任务计划程序设置脚本

# 获取项目根目录（脚本的上级目录）
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectPath = Split-Path -Parent $scriptDir
$exePath = Join-Path $projectPath "target\release\project.exe"

# 显示标题
Write-Host "`n================================================" -ForegroundColor Cyan
Write-Host "  TodoList 提醒功能 - 任务计划程序设置" -ForegroundColor Cyan
Write-Host "================================================`n" -ForegroundColor Cyan

# 检查可执行文件是否存在
if (-Not (Test-Path $exePath)) {
    Write-Host "❌ 错误: 找不到可执行文件 $exePath" -ForegroundColor Red
    Write-Host "`n💡 请先运行以下命令编译项目:" -ForegroundColor Yellow
    Write-Host "   cd $projectPath" -ForegroundColor Yellow
    Write-Host "   cargo build --release" -ForegroundColor Yellow
    Write-Host "`n按任意键退出..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    exit 1
}

Write-Host "✓ 找到可执行文件: $exePath" -ForegroundColor Green
Write-Host ""

# 任务名称
$taskName = "TodoListReminder"

# 检查任务是否已存在
$existingTask = Get-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue

if ($existingTask) {
    Write-Host "⚠️  任务 '$taskName' 已存在" -ForegroundColor Yellow
    Write-Host "   是否删除并重新创建？(Y/N): " -ForegroundColor Yellow -NoNewline
    $confirm = Read-Host
    if ($confirm -eq 'Y' -or $confirm -eq 'y') {
        try {
            Unregister-ScheduledTask -TaskName $taskName -Confirm:$false
            Write-Host "✓ 已删除旧任务" -ForegroundColor Green
        } catch {
            Write-Host "❌ 删除任务失败: $_" -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "操作已取消" -ForegroundColor Yellow
        Write-Host "`n按任意键退出..." -ForegroundColor Gray
        $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
        exit 0
    }
}

Write-Host "`n配置任务计划..." -ForegroundColor Cyan

# 询问执行频率
Write-Host "`n请选择提醒检查频率:" -ForegroundColor Yellow
Write-Host "  1. 每小时执行一次 (推荐)" -ForegroundColor White
Write-Host "  2. 每30分钟执行一次" -ForegroundColor White
Write-Host "  3. 每15分钟执行一次" -ForegroundColor White
Write-Host "  4. 自定义间隔(分钟)" -ForegroundColor White
Write-Host "请选择 (1-4): " -ForegroundColor Yellow -NoNewline
$choice = Read-Host

$intervalMinutes = 60
switch ($choice) {
    "1" { $intervalMinutes = 60 }
    "2" { $intervalMinutes = 30 }
    "3" { $intervalMinutes = 15 }
    "4" {
        Write-Host "请输入间隔分钟数 (1-1440): " -ForegroundColor Yellow -NoNewline
        $customInterval = Read-Host
        if ($customInterval -match '^\d+$' -and [int]$customInterval -ge 1 -and [int]$customInterval -le 1440) {
            $intervalMinutes = [int]$customInterval
        } else {
            Write-Host "❌ 无效的输入，使用默认值60分钟" -ForegroundColor Red
            $intervalMinutes = 60
        }
    }
    default {
        Write-Host "❌ 无效的选择，使用默认值60分钟" -ForegroundColor Red
        $intervalMinutes = 60
    }
}

Write-Host "✓ 已设置为每 $intervalMinutes 分钟执行一次" -ForegroundColor Green

# 创建任务操作 - Release 版本是 GUI 程序，可以直接运行，不会显示控制台
$action = New-ScheduledTaskAction `
    -Execute $exePath `
    -Argument "--check-reminders" `
    -WorkingDirectory $projectPath

# 创建触发器（每N分钟执行一次）
$trigger = New-ScheduledTaskTrigger `
    -Once `
    -At (Get-Date) `
    -RepetitionInterval (New-TimeSpan -Minutes $intervalMinutes)

# 创建任务设置（隐藏运行）
$settings = New-ScheduledTaskSettingsSet `
    -AllowStartIfOnBatteries `
    -DontStopIfGoingOnBatteries `
    -StartWhenAvailable `
    -RunOnlyIfNetworkAvailable:$false `
    -ExecutionTimeLimit (New-TimeSpan -Minutes 5) `
    -RestartCount 3 `
    -RestartInterval (New-TimeSpan -Minutes 1) `
    -Hidden

# 创建任务主体（当前用户权限）
$principal = New-ScheduledTaskPrincipal `
    -UserId $env:USERNAME `
    -LogonType Interactive `
    -RunLevel Limited

# 注册任务
try {
    Register-ScheduledTask `
        -TaskName $taskName `
        -Action $action `
        -Trigger $trigger `
        -Settings $settings `
        -Principal $principal `
        -Description "TodoList 自动提醒检查任务 - 每${intervalMinutes}分钟检查一次待办事项并发送提醒" `
        -ErrorAction Stop | Out-Null
    
    Write-Host "`n✅ 任务创建成功！" -ForegroundColor Green
} catch {
    Write-Host "`n❌ 任务创建失败: $_" -ForegroundColor Red
    Write-Host "`n按任意键退出..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    exit 1
}

# 显示任务信息
Write-Host "`n================================================" -ForegroundColor Cyan
Write-Host "  任务配置信息" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  任务名称: $taskName" -ForegroundColor White
Write-Host "  执行程序: $exePath" -ForegroundColor White
Write-Host "  执行参数: --check-reminders" -ForegroundColor White
Write-Host "  工作目录: $projectPath" -ForegroundColor White
Write-Host "  触发频率: 每 $intervalMinutes 分钟" -ForegroundColor White
Write-Host "  执行账户: $env:USERNAME" -ForegroundColor White
Write-Host "================================================`n" -ForegroundColor Cyan

Write-Host "💡 提示:" -ForegroundColor Yellow
Write-Host "  • 可以在 '任务计划程序' 中查看和管理此任务" -ForegroundColor Gray
Write-Host "  • Win + R 输入 'taskschd.msc' 打开任务计划程序" -ForegroundColor Gray
Write-Host "  • 在应用中可以通过 'reminder' 模式配置提醒功能" -ForegroundColor Gray

# 询问是否立即测试
Write-Host "`n是否立即测试运行一次？(Y/N): " -ForegroundColor Yellow -NoNewline
$testRun = Read-Host

if ($testRun -eq 'Y' -or $testRun -eq 'y') {
    Write-Host "`n正在测试运行..." -ForegroundColor Cyan
    try {
        Start-ScheduledTask -TaskName $taskName
        Start-Sleep -Seconds 2
        Write-Host "✓ 测试完成！请检查是否收到通知" -ForegroundColor Green
    } catch {
        Write-Host "⚠️  测试运行失败: $_" -ForegroundColor Yellow
    }
}

Write-Host "`n✅ 设置完成！提醒功能已启用" -ForegroundColor Green
Write-Host "`n按任意键退出..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

