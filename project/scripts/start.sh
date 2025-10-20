#!/bin/bash
# Todo List - 生产环境启动脚本 (Linux/macOS)

# 切换到项目根目录（脚本的上级目录）
cd "$(dirname "$0")/.." || exit 1

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
