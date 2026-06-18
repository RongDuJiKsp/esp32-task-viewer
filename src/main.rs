mod esp32_sys;
use esp32_sys::sys_init::SysInit;

slint::include_modules!();
fn main() {
    SysInit::init_sys();

    log::info!("Booting ESP32 Task Viewer...");
    let ui = App::new().unwrap();
    ui.run().unwrap();
}
