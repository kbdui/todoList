@echo off
chcp 65001 >nul
REM Todo List - 生产环境启动脚本 (Windows)

REM 切换到程序所在目录
cd /d "%~dp0"

REM 启动程序（Release 版本是 GUI 程序，需要通过 cmd /k 显示控制台）
cmd /k "project.exe"

REM 如果出错，保持窗口打开
if errorlevel 1 pause
