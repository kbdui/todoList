@echo off
chcp 65001 >nul
REM 自动编译和打包发布版本 (Windows)

echo ======================================
echo   Todo List - 自动打包脚本
echo ======================================
echo.

REM 切换到项目根目录（脚本的上级目录）
cd /d "%~dp0\.."

REM 1. 清理旧的构建
echo [1/5] 清理旧的构建...
cargo clean
if errorlevel 1 (
    echo 清理失败！
    pause
    exit /b 1
)
echo 完成！
echo.

REM 2. 编译 Release 版本
echo [2/5] 编译 Release 版本...
cargo build --release
if errorlevel 1 (
    echo 编译失败！
    pause
    exit /b 1
)
echo 完成！
echo.

REM 3. 创建发布目录
echo [3/5] 创建发布目录...
if exist "release-package" rd /s /q "release-package"
mkdir "release-package"
mkdir "release-package\database"
echo 完成！
echo.

REM 4. 复制文件
echo [4/5] 复制文件到发布目录...
copy "target\release\project.exe" "release-package\" >nul
copy "config.toml" "release-package\" >nul
copy "database\.gitkeep" "release-package\database\" >nul
if exist "document\使用说明.txt" copy "document\使用说明.txt" "release-package\" >nul
if exist "document\部署说明.md" copy "document\部署说明.md" "release-package\" >nul

REM 创建发布版启动脚本 (Windows)
(
echo @echo off
echo chcp 65001 ^>nul
echo REM Todo List - 启动脚本
echo.
echo REM 切换到程序所在目录
echo cd /d "%%~dp0"
echo.
echo REM 启动程序
echo project.exe
echo.
echo REM 如果出错，保持窗口打开
echo if errorlevel 1 pause
) > "release-package\start.bat"

REM 创建发布版启动脚本 (Linux/macOS)
(
echo #!/bin/bash
echo # Todo List - 启动脚本
echo.
echo # 切换到程序所在目录
echo cd "$$(dirname "$$0")" ^|^| exit 1
echo.
echo # 启动程序
echo ./project
echo.
echo # 捕获退出代码
echo exit_code=$$?
echo if [ $$exit_code -ne 0 ]; then
echo     echo ""
echo     echo "程序退出，错误代码: $$exit_code"
echo     read -p "按 Enter 键继续..."
echo fi
echo.
echo exit $$exit_code
) > "release-package\start.sh"

echo 完成！
echo.

REM 5. 创建压缩包
echo [5/5] 创建压缩包...
powershell -Command "Compress-Archive -Path 'release-package\*' -DestinationPath 'TodoList-Release.zip' -Force"
if errorlevel 1 (
    echo 打包失败！建议手动打包 release-package 目录
) else (
    echo 完成！
)
echo.

echo ======================================
echo   打包完成！
echo ======================================
echo.
echo 发布文件位置：
echo   目录：%~dp0..\release-package
echo   压缩包：%~dp0..\TodoList-Release.zip
echo.
echo 可以将 release-package 目录或 TodoList-Release.zip 发给用户
echo.

pause


