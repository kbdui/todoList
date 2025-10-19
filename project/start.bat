@echo off
REM Todo List - 生产环境启动脚本 (Windows)

REM 切换到脚本所在目录
cd /d "%~dp0"

REM 启动程序
project.exe

REM 如果出错，保持窗口打开
if errorlevel 1 pause

