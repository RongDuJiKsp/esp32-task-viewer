# esp32s3-battery-monitor

ESP32-S3 电池电量监控驱动库。

## 简介

本库为 ESP32-S3 提供电池电压采集与电量估算功能，基于 ADC 读取电池电压，并通过 `battery-estimator` 库进行电量百分比估算。

## 功能特性

- 通过 ESP32-S3 ADC 采集电池电压
- 支持多种电池化学类型（`BatteryChemistry`）
- 提供电量百分比估算
- 简洁的 IO 抽象层

## 公开 API

- `BatteryMonitor` — 电池监控器，负责电压采集与电量计算
- `BatteryIO` — 电池 ADC 引脚 IO 配置
- `BatteryChemistry` — 电池化学类型（来自 `battery-estimator`）

## 使用示例

```rust
use esp32s3_battery_monitor::{BatteryIO, BatteryMonitor, BatteryChemistry};

let io = BatteryIO::new(/* ADC 引脚配置 */);
let monitor = BatteryMonitor::new(io, BatteryChemistry::LiPo);
let percentage = monitor.read_percentage()?;
```

## 依赖

- `esp-idf-hal` — ESP-IDF HAL 抽象
- `battery-estimator` — 电池电量估算算法
- `anyhow` — 错误处理

## 许可证

MIT
