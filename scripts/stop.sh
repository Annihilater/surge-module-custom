#!/bin/bash

# 停止服务
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
PID_FILE="$PROJECT_DIR/tmp/surge-module-server.pid"

if [ ! -f "$PID_FILE" ]; then
    echo "PID file not found. Service may not be running."
    exit 1
fi

PID=$(cat "$PID_FILE")

if ! ps -p "$PID" > /dev/null 2>&1; then
    echo "Service is not running (PID: $PID not found)"
    rm -f "$PID_FILE"
    exit 1
fi

echo "Stopping surge-module-server (PID: $PID)..."
kill "$PID"

# 等待进程结束
for i in {1..10}; do
    if ! ps -p "$PID" > /dev/null 2>&1; then
        echo "Service stopped successfully"
        rm -f "$PID_FILE"
        exit 0
    fi
    sleep 1
done

# 如果还没停止，强制杀死
echo "Service did not stop gracefully, forcing..."
kill -9 "$PID"
rm -f "$PID_FILE"
echo "Service force stopped"
