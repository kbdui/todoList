@echo off
REM Todo List 启动脚本 (Windows)
REM 此脚本确保程序从正确的目录启动

REM 切换到项目根目录（脚本的上级目录）
cd /d "%~dp0\.."

REM 检查JSON配置中提醒功能是否启用
for /f "tokens=*" %%a in ('powershell -Command "$json = Get-Content 'database\config.json' -Raw -Encoding UTF8 | ConvertFrom-Json; if ($json.reminder.enabled -eq $true) { Write-Output 'TRUE' } else { Write-Output 'FALSE' }" 2^>nul') do set CHECK_RESULT=%%a
if "%CHECK_RESULT%"=="TRUE" (set REMINDER_ENABLED=0) else (set REMINDER_ENABLED=1)

REM 检查提醒任务是否已设置
powershell -Command "Get-ScheduledTask -TaskName 'TodoListReminder' -ErrorAction SilentlyContinue" >nul 2>&1
set TASK_EXISTS=%errorlevel%

REM 如果任务存在，检查任务路径是否与当前路径匹配
if %TASK_EXISTS% equ 0 (
    powershell -Command "$task = Get-ScheduledTask -TaskName 'TodoListReminder' -ErrorAction SilentlyContinue; $action = $task.Actions[0]; $scriptDir = Split-Path -Parent '%~f0'; $projectPath = Split-Path -Parent $scriptDir; $currentExePath = Join-Path $projectPath 'target\release\project.exe'; if (-Not (Test-Path $currentExePath)) { $currentExePath = Join-Path $projectPath 'target\debug\project.exe' }; if ($action.Execute -ne $currentExePath) { exit 1 } else { exit 0 }" >nul 2>&1
    if %errorlevel% neq 0 (
        REM 任务路径不匹配，需要重建
        echo.
        echo 检测到项目路径已改变，正在更新定时任务...
        
        powershell -Command "Unregister-ScheduledTask -TaskName 'TodoListReminder' -Confirm:$false -ErrorAction SilentlyContinue" >nul 2>&1
        set TASK_EXISTS=1
        echo 旧任务已删除，将重新创建
        echo.
    )
)

REM 根据配置和任务状态进行处理
if %REMINDER_ENABLED% equ 0 (
    REM 提醒功能已启用
    if %TASK_EXISTS% neq 0 (
        REM 任务不存在或路径不匹配，创建任务
        echo.
        echo 检测到提醒功能已启用，但未设置定时任务
        echo 正在自动设置定时任务...
        echo.
        
        powershell -ExecutionPolicy Bypass -File "%~dp0create_task.ps1"
        
        if %errorlevel% equ 0 (
            echo 提醒功能已启用
        ) else (
            echo 自动设置失败，您可以稍后手动设置
        )
        echo.
    )
) else (
    REM 提醒功能已禁用
    if %TASK_EXISTS% equ 0 (
        REM 任务存在，删除任务
        echo.
        echo 检测到提醒功能已禁用，正在删除定时任务...
        
        powershell -Command "Unregister-ScheduledTask -TaskName 'TodoListReminder' -Confirm:$false -ErrorAction Stop" >nul 2>&1
        
        if %errorlevel% equ 0 (
            echo 定时任务已删除
        ) else (
            echo 删除任务失败，您可以手动在任务计划程序中删除
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
