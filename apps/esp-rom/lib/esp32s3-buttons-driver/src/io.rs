/// 按钮引脚映射。
///
/// 所有按钮按下时接地（低电平有效）。
pub struct ButtonsIO<'a> {
    pub boot: esp_idf_hal::gpio::Gpio0<'a>,
    pub key: esp_idf_hal::gpio::Gpio18<'a>,
}
