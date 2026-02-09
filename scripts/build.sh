#!/bin/bash

# 构建项目
echo "Building surge-module-server..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi
