@echo off
REM Todo List 启动脚本 (Windows)
REM 此脚本确保程序从正确的目录启动

REM 切换到项目根目录（脚本的上级目录）
cd /d "%~dp0\.."

REM 检查提醒任务是否已设置
powershell -Command "Get-ScheduledTask -TaskName 'TodoListReminder' -ErrorAction SilentlyContinue" >nul 2>&1
if %errorlevel% neq 0 (
    REM 检查JSON配置中提醒功能是否启用
    powershell -Command "$json = Get-Content 'database\config.json' -Raw -ErrorAction SilentlyContinue | ConvertFrom-Json; if ($json.reminder.enabled -eq $true) { exit 0 } else { exit 1 }" >nul 2>&1
    if %errorlevel% equ 0 (
        echo.
        echo ⏰ 检测到提醒功能已启用，但未设置定时任务
        echo 💡 正在自动设置定时任务...
        echo.
        
        REM 自动设置定时任务（使用默认参数：每小时执行一次）
        powershell -ExecutionPolicy Bypass -Command "$scriptDir = Split-Path -Parent '%~f0'; $projectPath = Split-Path -Parent $scriptDir; $exePath = Join-Path $projectPath 'target\release\project.exe'; if (-Not (Test-Path $exePath)) { $exePath = Join-Path $projectPath 'target\debug\project.exe' }; if (Test-Path $exePath) { $action = New-ScheduledTaskAction -Execute $exePath -Argument '--check-reminders' -WorkingDirectory $projectPath; $trigger = New-ScheduledTaskTrigger -Once -At (Get-Date) -RepetitionInterval (New-TimeSpan -Minutes 60) -RepetitionDuration ([TimeSpan]::MaxValue); $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable -RunOnlyIfNetworkAvailable:$false -ExecutionTimeLimit (New-TimeSpan -Minutes 5) -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 1); $principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Limited; Register-ScheduledTask -TaskName 'TodoListReminder' -Action $action -Trigger $trigger -Settings $settings -Principal $principal -Description 'TodoList 自动提醒检查任务 - 每60分钟检查一次待办事项并发送提醒' -ErrorAction Stop | Out-Null; Write-Host '✅ 定时任务设置成功！' -ForegroundColor Green } else { Write-Host '❌ 找不到可执行文件，请先编译项目' -ForegroundColor Red; exit 1 }"
        
        if %errorlevel% equ 0 (
            echo ✅ 提醒功能已启用（每小时检查一次）
        ) else (
            echo ⚠️  自动设置失败，您可以稍后手动设置
        )
        echo.
    )
)

REM 检查可执行文件是否存在
if not exist "target\release\project.exe" (
    if not exist "target\debug\project.exe" (
        echo 错误：找不到可执行文件！
        echo 请先运行 "cargo build --release" 编译项目
        pause
        exit /b 1
    ) else (
        echo 使用 Debug 版本运行...
        target\debug\project.exe
    )
) else (
    target\release\project.exe
)

REM 保持窗口打开（如果程序异常退出）
if errorlevel 1 (
    echo.
    echo 程序异常退出，错误代码: %errorlevel%
    pause
)
