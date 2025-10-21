@echo off
chcp 65001 >nul
REM Todo List 启动脚本

cd /d "%~dp0\.."

REM 检查提醒功能是否启用
for /f "tokens=*" %%a in ('powershell -Command "$json = Get-Content 'database\config.json' -Raw -Encoding UTF8 | ConvertFrom-Json; if ($json.reminder.enabled -eq $true) { Write-Output 'TRUE' } else { Write-Output 'FALSE' }" 2^>nul') do set CHECK_RESULT=%%a
if "%CHECK_RESULT%"=="TRUE" (set REMINDER_ENABLED=0) else (set REMINDER_ENABLED=1)

REM 检查提醒任务是否已设置
powershell -Command "Get-ScheduledTask -TaskName 'TodoListReminder' -ErrorAction SilentlyContinue" >nul 2>&1
set TASK_EXISTS=%errorlevel%

REM 如果任务存在且提醒已启用，检查配置是否改变
set NEED_REBUILD=0
if %TASK_EXISTS% equ 0 if %REMINDER_ENABLED% equ 0 (
    powershell -ExecutionPolicy Bypass -File "%~dp0check_config_changed.ps1"
    if errorlevel 1 set NEED_REBUILD=1
)

REM 如果需要重建任务
if %NEED_REBUILD% equ 1 (
    echo.
    echo 检测到提醒配置已改变，正在更新定时任务...
    powershell -Command "Unregister-ScheduledTask -TaskName 'TodoListReminder' -Confirm:$false -ErrorAction SilentlyContinue" >nul 2>&1
    set TASK_EXISTS=1
    echo 旧任务已删除，将重新创建
    echo.
)

REM 处理提醒功能已启用的情况
if %REMINDER_ENABLED% equ 0 if %TASK_EXISTS% neq 0 (
    echo.
    echo 检测到提醒功能已启用，但未设置定时任务
    echo 正在自动设置定时任务...
    echo.
    powershell -ExecutionPolicy Bypass -File "%~dp0create_task.ps1"
    if errorlevel 1 (
        echo 自动设置失败，您可以稍后手动设置
    ) else (
        echo 提醒功能已启用
    )
    echo.
)

REM 处理提醒功能已禁用的情况
if %REMINDER_ENABLED% neq 0 if %TASK_EXISTS% equ 0 (
    echo.
    echo 检测到提醒功能已禁用，正在删除定时任务...
    powershell -Command "Unregister-ScheduledTask -TaskName 'TodoListReminder' -Confirm:$false -ErrorAction Stop" >nul 2>&1
    if errorlevel 1 (
        echo 删除任务失败，您可以手动在任务计划程序中删除
    ) else (
        echo 定时任务已删除
    )
    echo.
)

REM 启动程序
if not exist "target\debug\project.exe" (
    if not exist "target\release\project.exe" (
        echo 错误：找不到可执行文件！
        echo 请先运行 "cargo build" 编译项目
        pause
        exit /b 1
    ) else (
        cmd /k "target\release\project.exe"
    )
) else (
    target\debug\project.exe
)

REM 保持窗口打开
if errorlevel 1 (
    echo.
    echo 程序异常退出，错误代码: %errorlevel%
    pause
)
