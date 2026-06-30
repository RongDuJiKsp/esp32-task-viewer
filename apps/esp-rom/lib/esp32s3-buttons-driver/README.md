# esp32s3-buttons-driver

ESP32-S3 按键驱动库。

## 简介

本库为 ESP32-S3 提供按键输入驱动，支持按键状态读取与事件查看，适用于嵌入式 UI 交互场景。

## 功能特性

- GPIO 按键状态读取
- 按键事件抽象（`Button`）
- 按键查看器（`ButtonViewer`）用于轮询按键状态
- 简洁的 IO 抽象层

## 公开 API

- `Button` — 按键抽象，表示单个按键及其状态
- `ButtonsIO` — 按键 GPIO 引脚 IO 配置
- `ButtonViewer` — 按键查看器，用于读取和管理按键事件

## 使用示例

```rust
use esp32s3_buttons_driver::{ButtonsIO, ButtonViewer};

let io = ButtonsIO::new(/* GPIO 引脚配置 */);
let viewer = ButtonViewer::new(io);
let button_state = viewer.read()?;
```

## 依赖

- `esp-idf-hal` — ESP-IDF HAL 抽象
- `anyhow` — 错误处理

## 许可证

MIT
