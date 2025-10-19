#!/bin/bash
# 自动编译和打包发布版本 (Linux/macOS)

echo "======================================"
echo "  Todo List - 自动打包脚本"
echo "======================================"
echo ""

# 切换到脚本所在目录
cd "$(dirname "$0")" || exit 1

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
cp start.bat release-package/
cp start.sh release-package/
chmod +x release-package/start.sh
cp database/.gitkeep release-package/database/
cp 使用说明.txt release-package/ 2>/dev/null || echo "警告：使用说明.txt 不存在"
cp 部署说明.md release-package/ 2>/dev/null || echo "警告：部署说明.md 不存在"
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

