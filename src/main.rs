mod esp32_sys;
use std::panic;

use esp32_sys::{
    panic_handler::{ PanicHandler, PanicHandlerIO},
    sys_init::SysInit,
    display_raw::DisplayIO
};
use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};

slint::include_modules!();
fn main() {
    SysInit::init_patches();
    SysInit::init_logger();

    log::info!("Initializing peripherals...");
    let peripherals =
        Peripherals::take().expect("Failed to take peripherals");
    log::info!("Peripherals initialized successfully");

    let panic_handler_io =
        PinDriver::input(peripherals.pins.gpio0, Pull::Up)
            .unwrap();
    let panic_handler = PanicHandler::new(PanicHandlerIO::new(panic_handler_io));
    let panic_handler_ref = Box::leak(Box::new(panic_handler));
    panic::set_hook(Box::new(|info| {
        panic_handler_ref.handle_panic(info);
    }));

    SysInit::init_display(DisplayIO {
        spi: peripherals.spi2,
        sclk: peripherals.pins.gpio11,
        mosi: peripherals.pins.gpio12,
        cs: peripherals.pins.gpio40,
        dc: peripherals.pins.gpio5,
        rst: peripherals.pins.gpio41,
    });
    log::info!("Booting ESP32 Task Viewer...");
    let ui = App::new().unwrap();
    ui.run().unwrap();
}
