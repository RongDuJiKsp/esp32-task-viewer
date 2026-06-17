use anyhow::{Context, Result};
use core::time::Duration;
use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};
use std::{panic::PanicHookInfo, thread::sleep};

pub struct PanicHandler;
impl PanicHandler {
    pub fn handle_panic(info: &PanicHookInfo) {
        if let Err(err) = PanicHandler::try_handle_panic(info) {
            log::error!("Failed to handle panic: {err:#}");
        }
        PanicHandler::wait_forever();
    }

    fn wait_forever() -> ! {
        loop {
            sleep(Duration::from_secs(5));
        }
    }

    fn try_handle_panic(info: &PanicHookInfo) -> Result<()> {
        log::error!("Panic occurred: {}", info);
        PanicHandler::wait_boot_press()?;
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
