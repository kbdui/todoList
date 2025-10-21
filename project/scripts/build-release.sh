#!/bin/bash
# 自动编译和打包发布版本 (Linux/macOS)

echo "======================================"
echo "  Todo List - 自动打包脚本"
echo "======================================"
echo ""

# 切换到项目根目录（脚本的上级目录）
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR/.." || exit 1

# 1. 清理旧的构建
echo "[1/5] 清理旧的构建..."
cargo clean
if [ $? -ne 0 ]; then
    echo "清理失败！"
    exit 1
fi
echo "完成！"
echo ""

# 2. 编译 Release 版本
echo "[2/5] 编译 Release 版本..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "编译失败！"
    exit 1
fi
echo "完成！"
echo ""

# 3. 创建发布目录
echo "[3/5] 创建发布目录..."
rm -rf release-package
mkdir -p release-package/database
echo "完成！"
echo ""

# 4. 复制文件
echo "[4/5] 复制文件到发布目录..."
cp target/release/project release-package/
chmod +x release-package/project
cp config.toml release-package/
cp database/.gitkeep release-package/database/
[ -f "document/使用说明.txt" ] && cp document/使用说明.txt release-package/ || echo "警告：使用说明.txt 不存在"
[ -f "document/部署说明.md" ] && cp document/部署说明.md release-package/ || echo "警告：部署说明.md 不存在"

# 创建发布版启动脚本 (Windows)
cat > release-package/start.bat << 'EOF'
@echo off
chcp 65001 >nul
REM Todo List - 启动脚本

REM 切换到程序所在目录
cd /d "%~dp0"

REM 启动程序
project.exe

REM 如果出错，保持窗口打开
if errorlevel 1 pause
EOF

# 创建发布版启动脚本 (Linux/macOS)
cat > release-package/start.sh << 'EOF'
#!/bin/bash
# Todo List - 启动脚本

# 切换到程序所在目录
cd "$(dirname "$0")" || exit 1

# 启动程序
./project

# 捕获退出代码
exit_code=$?
if [ $exit_code -ne 0 ]; then
    echo ""
    echo "程序退出，错误代码: $exit_code"
    read -p "按 Enter 键继续..."
fi

exit $exit_code
EOF

chmod +x release-package/start.sh

echo "完成！"
echo ""

# 5. 创建压缩包
echo "[5/5] 创建压缩包..."
tar -czf TodoList-Release.tar.gz -C release-package .
if [ $? -ne 0 ]; then
    echo "打包失败！建议手动打包 release-package 目录"
else
    echo "完成！"
fi
echo ""

echo "======================================"
echo "  打包完成！"
echo "======================================"
echo ""
echo "发布文件位置："
echo "  目录：$(pwd)/release-package"
echo "  压缩包：$(pwd)/TodoList-Release.tar.gz"
echo ""
echo "可以将 release-package 目录或压缩包发给用户"
echo ""


