use anyhow::Result;
use core::{result::Result::Ok, time::Duration};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
};
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

use esp_idf_hal::gpio::PinDriver;
use std::{panic::PanicHookInfo, thread::sleep};

use esp32s3_st7305_lcd_display::{
    DisplayRaw, ESP32S3_LCP4_2_SCREEN_HEIGHT, ESP32S3_LCP4_2_SCREEN_WIDTH,
};
use std::sync::Arc;

pub struct PanicHandlerIO<'a> {
    boot_btn: PinDriver<'a, esp_idf_hal::gpio::Input>,
}
impl<'a> PanicHandlerIO<'a> {
    pub fn new(boot_btn: PinDriver<'a, esp_idf_hal::gpio::Input>) -> Self {
        PanicHandlerIO { boot_btn }
    }
}

pub struct PanicHandler<'a> {
    inner: PanicHandlerInner<'a>,
}
struct PanicHandlerInner<'a> {
    io: PanicHandlerIO<'a>,
    display: Arc<DisplayRaw>,
}
// wrap
impl<'a> PanicHandler<'a> {
    pub fn handle_panic(&self, info: &PanicHookInfo) {
        if let Err(err) = self.inner.try_handle_panic(info) {
            log::error!("Failed to handle panic: {err:#}");
        }
        PanicHandler::wait_forever();
    }

    pub fn new(io: PanicHandlerIO<'a>, display: Arc<DisplayRaw>) -> Self {
        let inner = PanicHandlerInner::new(io, display);
        PanicHandler { inner }
    }

    fn wait_forever() -> ! {
        loop {
            sleep(Duration::from_secs(5));
        }
    }
}

// actual implementation
impl<'a> PanicHandlerInner<'a> {
    fn new(io: PanicHandlerIO<'a>, display: Arc<DisplayRaw>) -> Self {
        PanicHandlerInner { io, display }
    }
    fn try_handle_panic(&self, info: &PanicHookInfo) -> Result<()> {
        log::error!("Panic occurred: {}", info);
        self.print_panic_info_to_lcd(info)?;
        self.wait_boot_press()?;
        Ok(())
    }
    fn print_panic_info_to_lcd(&self, info: &PanicHookInfo) -> Result<()> {
        let mut screen = self.display.get_display()?;

        let text = format!("SYSTEM PANIC !!!\n\n{}", info);
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .build();
        let margin = 10u32;
        let area = Rectangle::new(
            Point::new(margin as i32, margin as i32),
            Size::new(
                ESP32S3_LCP4_2_SCREEN_WIDTH - 2 * margin,
                ESP32S3_LCP4_2_SCREEN_HEIGHT - 2 * margin,
            ),
        );
        TextBox::with_textbox_style(&text, area, character_style, textbox_style)
            .draw(&mut (*screen))?;

        screen
            .flush()
            .map_err(|e| anyhow::anyhow!("Failed to flush display: {:#?}", e))?;

        Ok(())
    }
    fn wait_boot_press(&self) -> Result<()> {
        log::info!("Press the BOOT button to continue...");
        loop {
            if self.io.boot_btn.is_low() {
                log::info!("BOOT button pressed. Restarting...");
                unsafe {
                    esp_idf_sys::esp_restart();
                }
            }
            sleep(Duration::from_secs(3));
        }
    }
}
