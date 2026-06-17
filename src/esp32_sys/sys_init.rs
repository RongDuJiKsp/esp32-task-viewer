use std::sync::{Mutex, OnceLock};

use crate::esp32_sys::display_raw::{DisplayIO, DisplayRaw};

pub static GLOBAL_DISPLAY: OnceLock<Mutex<Box<DisplayRaw<'static>>>> = OnceLock::new();
pub struct SysInit;
impl SysInit {
    pub fn init_patches() {
        esp_idf_svc::sys::link_patches();
    }
    pub fn init_logger() {
        esp_idf_svc::log::EspLogger::initialize_default();
    }
    pub fn init_display(display_pin: DisplayIO<'static>) {
        log::info!("Initializing display...");
        let mut display = DisplayRaw::new(display_pin).unwrap();
        display.init().unwrap();
        log::info!("Display initialized successfully");
        GLOBAL_DISPLAY.set(Mutex::new(Box::new(display))).unwrap();
        log::info!("Display set in GLOBAL_DISPLAY");
    }
}
