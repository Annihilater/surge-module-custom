#!/bin/bash

# 前台启动服务（开发模式）
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Starting surge-module-server in foreground (dev mode)..."
cd "$PROJECT_DIR"
cargo run
