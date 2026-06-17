mod esp32_sys;
use std::panic;

use esp32_sys::panic_handler::PanicHandler;
use esp32_sys::sys_init::SysInit;
slint::include_modules!();
fn main() {
    SysInit::init_patches();
    SysInit::init_logger();
    panic::set_hook(Box::new(|info| {
        PanicHandler::handle_panic(info);
    }));
    log::info!("Booting ESP32 Task Viewer...");
    let ui = App::new().unwrap();
    ui.run().unwrap();
}
