#!/bin/bash

# 重启服务
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Restarting surge-module-server..."

# 停止服务
"$SCRIPT_DIR/stop.sh"

# 等待一秒
sleep 1

# 启动服务
"$SCRIPT_DIR/start.sh"
