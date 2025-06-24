# WakeUp - Wake on LAN 工具

一个用 Rust 编写的 Wake on LAN 工具，支持配置文件管理和主机快速唤醒。

## 功能特性

- 🚀 发送 Wake on LAN 魔法包唤醒主机
- 📝 配置文件管理（config.toml）
- 🎯 支持自定义广播地址
- 📋 列出已配置的主机
- ✨ 简单易用的命令行界面

## 安装

```bash
git clone <your-repo>
cd wakeup
cargo build --release
```

## 使用方法

### 添加主机配置

```bash
# 添加主机（使用默认广播地址 255.255.255.255:9）
./target/release/wakeup add mypc 00:11:22:33:44:55

# 添加主机并指定广播地址
./target/release/wakeup add server 00:AA:BB:CC:DD:EE --broadcast 192.168.1.255:9
```

### 唤醒主机

```bash
./target/release/wakeup wake mypc
```

### 列出所有配置的主机

```bash
./target/release/wakeup list
```

### 删除主机配置

```bash
./target/release/wakeup remove mypc
```

## 配置文件格式

配置文件 `config.toml` 会自动创建在程序运行目录下：

```toml
[hosts]

[hosts.mypc]
mac = "00:11:22:33:44:55"
broadcast = "255.255.255.255:9"

[hosts.server]
mac = "00:AA:BB:CC:DD:EE"
broadcast = "192.168.1.255:9"
```

## MAC 地址格式

MAC 地址必须使用冒号分隔的格式：`XX:XX:XX:XX:XX:XX`

例如：`00:11:22:33:44:55`

## 广播地址

默认广播地址是 `255.255.255.255:9`，但你可以根据网络配置指定不同的广播地址。

常用端口：
- 端口 9 (默认)
- 端口 7
- 端口 0

## 工作原理

Wake on LAN 通过发送一个特殊的网络包（魔法包）来唤醒支持 WoL 的设备。魔法包包含：
1. 6 个字节的 0xFF
2. 目标设备的 MAC 地址重复 16 次

## 注意事项

1. 目标设备必须支持并启用 Wake on LAN 功能
2. 目标设备必须连接到电源
3. 网络设备（路由器/交换机）必须支持广播包转发
4. 某些防火墙可能会阻止魔法包

## 许可证

MIT License
