#!/bin/bash

# 查看服务状态
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
PID_FILE="$PROJECT_DIR/tmp/surge-module-server.pid"

if [ ! -f "$PID_FILE" ]; then
    echo "Service is not running (PID file not found)"
    exit 1
fi

PID=$(cat "$PID_FILE")

if ps -p "$PID" > /dev/null 2>&1; then
    echo "Service is running"
    echo "PID: $PID"
    echo ""
    ps -p "$PID" -o pid,ppid,user,%cpu,%mem,etime,command
else
    echo "Service is not running (PID: $PID not found)"
    rm -f "$PID_FILE"
    exit 1
fi
