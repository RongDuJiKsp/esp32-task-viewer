use std::{panic::PanicHookInfo, sync::Arc, time::Instant};

use anyhow::Result;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
};
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};
use esp32s3_buttons_driver::ButtonViewer;
use esp32s3_st7305_lcd_display::{
    DisplayRaw, ESP32S3_LCP4_2_SCREEN_HEIGHT, ESP32S3_LCP4_2_SCREEN_WIDTH,
};

pub struct PanicHandler {
    inner: PanicHandlerInner,
}
struct PanicHandlerInner {
    buttons: Arc<ButtonViewer>,
    display: Arc<DisplayRaw>,
}
impl PanicHandler {
    pub fn handle_panic(&self, info: &PanicHookInfo<'_>) {
        if let Err(err) = self.inner.try_handle_panic(info) {
            log::error!("Failed to handle panic: {err:#}");
        }
        PanicHandler::wait_forever();
    }

    pub fn new(buttons: Arc<ButtonViewer>, display: Arc<DisplayRaw>) -> Self {
        let inner = PanicHandlerInner::new(buttons, display);
        PanicHandler { inner }
    }

    fn wait_forever() -> ! {
        loop {
            core::hint::spin_loop();
        }
    }
}

// actual implementation
impl PanicHandlerInner {
    fn new(buttons: Arc<ButtonViewer>, display: Arc<DisplayRaw>) -> Self {
        PanicHandlerInner { buttons, display }
    }

    fn try_handle_panic(&self, info: &PanicHookInfo<'_>) -> Result<()> {
        log::error!("Panic occurred: {}", info);
        self.print_panic_info_to_lcd(info)?;
        self.wait_boot_press()?;
        Ok(())
    }

    fn print_panic_info_to_lcd(&self, info: &PanicHookInfo<'_>) -> Result<()> {
        let mut screen = self.display.get_display()?;

        let text = format!("SYSTEM PANIC !!!\n\n{}", info);
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new().alignment(HorizontalAlignment::Left).build();
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

        screen.flush().map_err(|e| anyhow::anyhow!("Failed to flush display: {:#?}", e))?;

        Ok(())
    }

    fn wait_boot_press(&self) -> Result<()> {
        log::info!("Press the BOOT button to continue...");
        loop {
            if self.buttons.button_raw()?.is_boot_pressed() {
                log::info!("BOOT button pressed. Restarting...");
                // SAFETY: esp_restart() performs a full chip reset.
                // Called only in panic context after displaying panic info to LCD,
                // as a final recovery mechanism triggered by user interaction.
                unsafe {
                    esp_idf_sys::esp_restart();
                }
            }
            let deadline = Instant::now() + core::time::Duration::from_millis(3000);
            while Instant::now() < deadline {
                core::hint::spin_loop();
            }
        }
    }
}
