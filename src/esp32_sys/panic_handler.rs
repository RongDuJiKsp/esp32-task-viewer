use core::time::Duration;
use esp_idf_hal::{
    gpio::{Input, PinDriver, Pull},
    peripherals::Peripherals,
};
use std::{panic::PanicHookInfo, thread::sleep};

pub struct PanicHandler;
impl PanicHandler {
    pub fn handle_panic(info: &PanicHookInfo) {
        log::error!("Panic occurred: {}", info);
        PanicHandler::wait_boot_press();
    }

    fn wait_boot_press() {
        log::info!("Press the BOOT button to continue...");
        let peripherals = Peripherals::take().unwrap();
        let boot = PinDriver::input(peripherals.pins.gpio0, Pull::Up).unwrap();
        loop {
            if boot.is_low() {
                unsafe {
                    esp_idf_sys::esp_restart();
                }
            }
            sleep(Duration::from_secs(1));
        }
    }
}
