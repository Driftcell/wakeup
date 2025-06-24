#!/bin/bash

# WakeUp 使用示例脚本

echo "=== WakeUp - Wake on LAN 工具使用示例 ==="
echo

# 构建项目
echo "1. 构建项目..."
cargo build --release
echo

# 显示帮助
echo "2. 显示帮助信息:"
./target/release/wakeup --help
echo

# 添加主机示例
echo "3. 添加主机配置示例:"
echo "   添加主机 'mypc' (MAC: 00:11:22:33:44:55)"
./target/release/wakeup add mypc 00:11:22:33:44:55

echo "   添加服务器 'server' (MAC: 00:AA:BB:CC:DD:EE, 自定义广播地址)"
./target/release/wakeup add server 00:AA:BB:CC:DD:EE --broadcast 192.168.1.255:9
echo

# 列出主机
echo "4. 列出所有配置的主机:"
./target/release/wakeup list
echo

# 显示配置文件内容
echo "5. 生成的配置文件内容 (config.toml):"
cat config.toml
echo

# 唤醒主机示例（注意：这会发送实际的网络包）
echo "6. 唤醒主机示例:"
echo "   使用命令: ./target/release/wakeup wake mypc"
echo "   （注意：这会发送实际的 Wake-on-LAN 包）"
echo

# 删除主机示例  
echo "7. 删除主机配置:"
./target/release/wakeup remove mypc
echo

echo "8. 确认删除后的主机列表:"
./target/release/wakeup list
echo

echo "=== 示例完成 ==="
echo "现在你可以使用以下命令管理你的主机:"
echo "  - 添加主机: ./target/release/wakeup add <name> <mac> [--broadcast <addr>]"
echo "  - 唤醒主机: ./target/release/wakeup wake <name>"
echo "  - 列出主机: ./target/release/wakeup list"
echo "  - 删除主机: ./target/release/wakeup remove <name>"
