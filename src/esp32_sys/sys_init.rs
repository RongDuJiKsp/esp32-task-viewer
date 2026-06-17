pub struct SysInit;
impl SysInit {
    pub fn init_patches() {
        esp_idf_svc::sys::link_patches();
    }
    pub fn init_logger() {
        esp_idf_svc::log::EspLogger::initialize_default();
    }
}
