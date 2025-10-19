@echo off
REM Todo List 启动脚本 (Windows)
REM 此脚本确保程序从正确的目录启动

REM 切换到脚本所在目录
cd /d "%~dp0"

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

