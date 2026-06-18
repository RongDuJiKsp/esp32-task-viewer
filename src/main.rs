mod esp32_sys;
use esp32_sys::{
    slint_st7305_platform::SlintSt7305Platform,
    sys_init::{SysInit, SysStore},
};
use slint::platform;

slint::include_modules!();
fn main() {
    SysInit::init_sys();

    log::info!("Booting ESP32 Task Viewer...");
    platform::set_platform(Box::new(SlintSt7305Platform::new(SysStore::get_display()))).unwrap();

    let ui = App::new().unwrap();
    ui.run().unwrap();
}
