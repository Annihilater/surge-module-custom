#!/bin/bash

# 后台启动服务
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
PID_FILE="$PROJECT_DIR/tmp/surge-module-server.pid"
LOG_FILE="$PROJECT_DIR/logs/surge-module-server.log"

# 创建必要的目录
mkdir -p "$PROJECT_DIR/tmp"
mkdir -p "$PROJECT_DIR/logs"

# 检查是否已经在运行
if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    if ps -p "$PID" > /dev/null 2>&1; then
        echo "Service is already running with PID: $PID"
        exit 1
    else
        echo "Removing stale PID file..."
        rm -f "$PID_FILE"
    fi
fi

# 构建项目
echo "Building surge-module-server..."
cd "$PROJECT_DIR"
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# 后台启动服务
echo "Starting surge-module-server in background..."
nohup "$PROJECT_DIR/target/release/surge-module-server" > "$LOG_FILE" 2>&1 &
PID=$!

# 保存 PID
echo $PID > "$PID_FILE"

echo "Service started successfully with PID: $PID"
echo "Log file: $LOG_FILE"
