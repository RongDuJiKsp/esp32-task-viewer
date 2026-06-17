use anyhow::{Context, Result};
use core::{result::Result::Ok, time::Duration};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};
use std::{panic::PanicHookInfo, thread::sleep};

use crate::esp32_sys::{display_raw::DisplayRaw, sys_init::GLOBAL_DISPLAY};
pub struct PanicHandler;
struct PanicHandlerInner;
// wrap
impl PanicHandler {
    pub fn handle_panic(info: &PanicHookInfo) {
        if let Err(err) = PanicHandlerInner::try_handle_panic(info) {
            log::error!("Failed to handle panic: {err:#}");
        }
        PanicHandler::wait_forever();
    }

    fn wait_forever() -> ! {
        loop {
            sleep(Duration::from_secs(5));
        }
    }
}

// actual implementation
impl PanicHandlerInner {
    fn try_handle_panic(info: &PanicHookInfo) -> Result<()> {
        log::error!("Panic occurred: {}", info);
        PanicHandlerInner::print_panic_info_to_lcd(info)?;
        PanicHandlerInner::wait_boot_press()?;
        Ok(())
    }
    fn print_panic_info_to_lcd(info: &PanicHookInfo) -> Result<()> {
        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        Text::new(info.payload_as_str().unwrap_or("Unknown panic"), Point::new(10, 30), style).draw(
            GLOBAL_DISPLAY
                .get()
                .ok_or_else(|| anyhow::anyhow!("Failed to get global display"))?
                .lock()
                .map_err(|e| anyhow::anyhow!("Failed to lock global display: {e}"))?
                .get_display_mut(),
        )?;
        Ok(())
    }
    fn wait_boot_press() -> Result<()> {
        log::info!("Press the BOOT button to continue...");
        let boot_pin = Peripherals::take()
            .and_then(|p| PinDriver::input(p.pins.gpio0, Pull::Up))
            .context("failed to initialize BOOT button pin")?;

        loop {
            if boot_pin.is_low() {
                log::info!("BOOT button pressed. Restarting...");
                unsafe {
                    esp_idf_sys::esp_restart();
                }
            }
            sleep(Duration::from_secs(3));
        }
    }
}
