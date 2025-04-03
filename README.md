# 环境监控系统 (env_monitor)

[![GitHub](https://img.shields.io/badge/github-zccccc01/env_monitor-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/zccccc01/env_monitor)
[![crates.io](https://img.shields.io/crates/v/env_monitor.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/env_monitor)
[![docs.rs](https://img.shields.io/badge/docs.rs-env_monitor-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/env_monitor)

使用 Raspberry Pi 等设备上的传感器来监控环境。它集成了 DHT11 温湿度传感器、火焰传感器和蜂鸣器。项目支持异步操作，利用 Tokio 实现并发数据读取和处理。

## 功能

- **DHT11 温湿度传感器**：读取当前环境的温度和湿度。
- **火焰传感器**：监测火灾，并在火焰被检测到时触发蜂鸣器报警。
- **蜂鸣器控制**：当火灾发生时，蜂鸣器发出警报。

## 安装

### 克隆项目

```bash
git clone https://github.com/zccccc01/env_monitor.git
cd env_monitor
```

### 安装依赖

在项目目录下，运行以下命令安装依赖：

```bash
cargo build
```

## 配置

确保树莓派 3b+已连接以下硬件：

- **DHT11 温湿度传感器**：连接到 GPIO 17 引脚
- **火焰传感器**：连接到 GPIO 27 引脚
- **蜂鸣器**：连接到 GPIO 22 引脚

### 启动示例

运行以下命令启动环境监控系统：

```bash
cargo run --example env_monitor_example
```
