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
    let ui_ref = ui.clone_strong();
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        core::time::Duration::from_millis(1000),
        move || {
            ui_ref.set_battery(75);
            let now=chrono::Local::now();
            ui_ref.set_current_time(now.format("%H:%M").to_string().into());
            ui_ref.set_current_second(now.format("%S").to_string().into());
            ui_ref.set_current_date(now.format("%Y-%m-%d").to_string().into());
        },
    );

    ui.run().unwrap();
}
