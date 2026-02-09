#!/bin/bash

# 停止服务
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
PID_FILE="$PROJECT_DIR/tmp/surge-module-server.pid"
PROCESS_NAME="surge-module-server"

# 函数：通过进程名查找 PID
find_pid_by_name() {
    pgrep -f "$PROCESS_NAME" | head -n 1
}

# 函数：停止进程
stop_process() {
    local pid=$1
    echo "Stopping $PROCESS_NAME (PID: $pid)..."
    kill "$pid"

    # 等待进程结束
    for i in {1..10}; do
        if ! ps -p "$pid" > /dev/null 2>&1; then
            echo "Service stopped successfully"
            return 0
        fi
        sleep 1
    done

    # 如果还没停止，强制杀死
    echo "Service did not stop gracefully, forcing..."
    kill -9 "$pid" 2>/dev/null
    echo "Service force stopped"
    return 0
}

# 优先使用 PID 文件
if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    if ps -p "$PID" > /dev/null 2>&1; then
        stop_process "$PID"
        rm -f "$PID_FILE"
        exit 0
    else
        echo "PID file exists but process not found, cleaning up..."
        rm -f "$PID_FILE"
    fi
fi

# 如果 PID 文件不存在或进程已不存在，尝试通过进程名查找
echo "Searching for $PROCESS_NAME process..."
PID=$(find_pid_by_name)

if [ -z "$PID" ]; then
    echo "Service is not running"
    exit 1
fi

stop_process "$PID"
rm -f "$PID_FILE"
