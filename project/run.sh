#!/bin/bash
# Todo List 启动脚本 (Linux/macOS)
# 此脚本确保程序从正确的目录启动

# 切换到脚本所在目录
cd "$(dirname "$0")" || exit 1

# 检查可执行文件是否存在
if [ -f "target/release/project" ]; then
    echo "使用 Release 版本运行..."
    ./target/release/project
elif [ -f "target/debug/project" ]; then
    echo "使用 Debug 版本运行..."
    ./target/debug/project
else
    echo "错误：找不到可执行文件！"
    echo "请先运行 'cargo build --release' 编译项目"
    exit 1
fi

# 捕获退出代码
exit_code=$?
if [ $exit_code -ne 0 ]; then
    echo ""
    echo "程序异常退出，错误代码: $exit_code"
    read -p "按 Enter 键继续..."
fi

exit $exit_code

