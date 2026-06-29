# esp32s3-slint-st7305-platform

基于 ST7305 显示屏的 Slint UI 平台后端，适用于 ESP32-S3。

## 简介

本库为 ESP32-S3 + ST7305 LCD 显示屏组合提供 Slint UI 框架的平台后端实现，使得 Slint 应用可以直接在该硬件平台上运行渲染。

## 功能特性

- 实现 Slint `Platform` trait，支持 ST7305 显示屏渲染
- 像素格式转换（Slint 像素 ↔ ST7305 显示格式）
- 基于 `esp32s3-st7305-lcd-display` 的显示输出
- 软件渲染器支持

## 公开 API

- `SlintSt7305Platform` — Slint 平台后端，注册后即可运行 Slint UI

## 使用示例

```rust
use esp32s3_slint_st7305_platform::SlintSt7305Platform;

let platform = SlintSt7305Platform::new(/* 显示屏实例 */);
slint::platform::set_platform(Box::new(platform)).unwrap();
// 之后正常运行 Slint UI
```

## 依赖

- `esp32s3-st7305-lcd-display` — ST7305 显示屏驱动
- `slint` — Slint UI 框架（软件渲染模式）
- `esp-idf-hal` — ESP-IDF HAL 抽象
- `embedded-graphics` — 嵌入式图形库

## 许可证

MIT
