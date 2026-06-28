mod esp32_sys;
use esp32_sys::sys_init::{SysInit, SysStore};
use esp32s3_slint_st7305_platform::SlintSt7305Platform;
use slint::platform;

slint::include_modules!();
fn main() {
    SysInit::init_sys();

    log::info!("Booting ESP32 Task Viewer...");
    platform::set_platform(Box::new(SlintSt7305Platform::new(SysStore::get_display()))).unwrap();

    let battery = SysStore::get_battery();
    let ui = App::new().unwrap();
    let ui_ref = ui.clone_strong();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, core::time::Duration::from_millis(1000), move || {
        // 电池电量与电压
        match battery.read_soc() {
            Ok(soc) => ui_ref.set_battery(i32::from(soc)),
            Err(e) => log::warn!("Failed to read battery SOC: {e}"),
        }
        match battery.read_voltage_v() {
            Ok(v) => ui_ref.set_voltage(format!("{:.1}V", v).into()),
            Err(e) => log::warn!("Failed to read battery voltage: {e}"),
        }

        // 时间
        let now = chrono::Local::now();
        ui_ref.set_current_hour(now.format("%H").to_string().into());
        ui_ref.set_current_minute(now.format("%M").to_string().into());
        ui_ref.set_current_second(now.format("%S").to_string().into());
        ui_ref.set_current_date(now.format("%Y-%m-%d").to_string().into());
        ui_ref.set_current_weekday(now.format("%A").to_string().into());
    });

    ui.run().unwrap();
}
