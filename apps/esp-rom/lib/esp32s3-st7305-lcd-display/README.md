# esp32s3-st7305-lcd-display

ESP32-S3 ST7305 LCD 显示屏驱动库。

## 简介

本库为 ESP32-S3 提供 ST7305 LCD 显示屏的底层驱动，支持 SPI 通信、显示初始化与帧缓冲写入，适用于 4.2 寸低功耗电子纸风格 LCD 屏幕（400×300 分辨率）。

## 功能特性

- ST7305 LCD 显示屏 SPI 驱动
- 支持 400×300 分辨率显示
- 提供原始帧缓冲写入接口（`DisplayRaw`）
- 封装高层显示接口（`St7305Display`）
- 简洁的 IO 抽象层

## 公开 API

- `St7305Display` — 高层显示接口，封装初始化与刷新逻辑
- `DisplayRaw` — 原始显示缓冲区操作
- `DisplayIO` — 显示屏 SPI 及控制引脚 IO 配置
- `ESP32S3_LCP4_2_SCREEN_WIDTH` — 屏幕宽度常量（400）
- `ESP32S3_LCP4_2_SCREEN_HEIGHT` — 屏幕高度常量（300）

## 使用示例

```rust
use esp32s3_st7305_lcd_display::{DisplayIO, St7305Display};

let io = DisplayIO::new(/* SPI 及引脚配置 */);
let mut display = St7305Display::new(io)?;
display.init()?;
// 写入帧数据并刷新
```

## 依赖

- `esp-idf-hal` — ESP-IDF HAL 抽象
- `st7305` — ST7305 驱动核心库
- `display-interface` / `display-interface-spi` — 显示接口抽象
- `anyhow` — 错误处理

## 许可证

MIT
