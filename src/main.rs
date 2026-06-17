mod esp32_sys;
use core::result::Result::Ok;
use std::panic;

use esp32_sys::panic_handler::PanicHandler;
use esp32_sys::sys_init::SysInit;

use crate::esp32_sys::display_raw::DisplayIO;
slint::include_modules!();
fn main() {
    SysInit::init_patches();
    SysInit::init_logger();

    log::info!("Initializing peripherals...");
    let peripherals =
        esp_idf_hal::peripherals::Peripherals::take().expect("Failed to take peripherals");
    log::info!("Peripherals initialized successfully");

    SysInit::init_display(DisplayIO {
        spi: peripherals.spi2,
        sclk: peripherals.pins.gpio11,
        mosi: peripherals.pins.gpio12,
        cs: peripherals.pins.gpio40,
        dc: peripherals.pins.gpio5,
        rst: peripherals.pins.gpio41,
    });

    panic::set_hook(Box::new(|info| {
        PanicHandler::handle_panic(info);
    }));

    log::info!("Booting ESP32 Task Viewer...");
    let ui = App::new().unwrap();
    ui.run().unwrap();
}
