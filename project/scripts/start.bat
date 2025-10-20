@echo off
REM Todo List - 生产环境启动脚本 (Windows)

REM 切换到项目根目录（脚本的上级目录）
cd /d "%~dp0\.."

REM 启动程序
project.exe

REM 如果出错，保持窗口打开
if errorlevel 1 pause
