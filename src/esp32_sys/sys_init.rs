use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{panic, sync::OnceLock};

use crate::esp32_sys::panic_handler::{PanicHandler, PanicHandlerIO};
use esp32s3_st7305_lcd_display::{DisplayIO, DisplayRaw};
use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};

static INIT_FLAG: AtomicBool = AtomicBool::new(false);
static GLOBAL_DISPLAY: OnceLock<Arc<DisplayRaw>> = OnceLock::new();
pub struct SysInit;
impl SysInit {
    pub fn init_sys() {
        if INIT_FLAG.load(Ordering::Relaxed) {
            panic!("Repeated Init")
        }
        Self::init_patches();
        Self::init_logger();
        Self::init_pins();
        INIT_FLAG.store(true, Ordering::Relaxed);
    }
    fn init_patches() {
        esp_idf_svc::sys::link_patches();
    }
    fn init_logger() {
        esp_idf_svc::log::EspLogger::initialize_default();
    }
    fn init_pins() {
        log::info!("Initializing peripherals...");
        let peripherals = Peripherals::take().expect("Failed to take peripherals");
        log::info!("Peripherals initialized successfully");

        let display_pin = DisplayIO {
            spi: peripherals.spi2,
            sclk: peripherals.pins.gpio11,
            mosi: peripherals.pins.gpio12,
            cs: peripherals.pins.gpio40,
            dc: peripherals.pins.gpio5,
            rst: peripherals.pins.gpio41,
        };
        log::info!("Initializing display...");
        let display = Arc::new(DisplayRaw::new(display_pin).unwrap());
        display.init().unwrap();
        log::info!("Display initialized successfully");

        let panic_handler_io = PinDriver::input(peripherals.pins.gpio0, Pull::Up).unwrap();
        let panic_handler =
            PanicHandler::new(PanicHandlerIO::new(panic_handler_io), display.clone());
        let panic_handler_ref = Box::leak(Box::new(panic_handler));
        panic::set_hook(Box::new(|info| {
            panic_handler_ref.handle_panic(info);
        }));

        GLOBAL_DISPLAY.set(display).unwrap();
    }
}

pub struct SysStore;
impl SysStore {
    pub fn get_display() -> Arc<DisplayRaw> {
        GLOBAL_DISPLAY
            .get()
            .expect("Display not initialized")
            .clone()
    }
}
