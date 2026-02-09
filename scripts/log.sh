#!/bin/bash

# 查看服务日志
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LOG_FILE="$PROJECT_DIR/logs/surge-module-server.log"

if [ ! -f "$LOG_FILE" ]; then
    echo "Log file not found: $LOG_FILE"
    exit 1
fi

# 如果有参数，使用参数作为 tail 的行数，否则默认显示最后 100 行
LINES=${1:-100}

echo "Showing last $LINES lines of log file:"
echo "----------------------------------------"
tail -n "$LINES" -f "$LOG_FILE"
