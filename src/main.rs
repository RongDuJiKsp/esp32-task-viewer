mod esp32_sys;
use esp32_sys::sys_init::SysInit;
fn main() {
    SysInit::init_patches();
    SysInit::init_logger();
    log::info!("Booting ESP32 Task Viewer...");
    loop {
        log::info!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
