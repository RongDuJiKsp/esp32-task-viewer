/*
连接关系：
ESP32-S3                 18650 Battery (via voltage divider)
---------                ----------
GPIO4  ------------->    ADC (battery voltage, 3:1 divider)
*/

pub struct BatteryIO<'a> {
    pub adc: esp_idf_hal::adc::ADC1<'a>,
    pub battery_pin: esp_idf_hal::gpio::Gpio4<'a>,
}
