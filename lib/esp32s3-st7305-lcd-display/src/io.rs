/*
连接关系：
ESP32-S3                 ST7305
---------                ------
GPIO11  ------------->   SCLK
GPIO12  ------------->   MOSI/DIN
GPIO40  ------------->   CS
GPIO5   ------------->   D/C
GPIO41  ------------->   RESET
*/
pub struct DisplayIO<'a> {
    pub spi: esp_idf_hal::spi::SPI2<'a>,
    pub sclk: esp_idf_hal::gpio::Gpio11<'a>,
    pub mosi: esp_idf_hal::gpio::Gpio12<'a>,
    pub cs: esp_idf_hal::gpio::Gpio40<'a>,
    pub dc: esp_idf_hal::gpio::Gpio5<'a>,
    pub rst: esp_idf_hal::gpio::Gpio41<'a>,
}
