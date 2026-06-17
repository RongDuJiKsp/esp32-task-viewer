#![allow(dead_code)]
use core::fmt::Debug;

use anyhow::Result;
use display_interface_spi::SPIInterface;
use esp_idf_hal::{
    gpio::{AnyIOPin, PinDriver},
    spi::{SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use st7305::St7305;
type St7305Display<'a> = St7305<
    SPIInterface<SpiDeviceDriver<'a, SpiDriver<'a>>, PinDriver<'a, esp_idf_hal::gpio::Output>>,
    PinDriver<'a, esp_idf_hal::gpio::Output>,
>;
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
pub struct DisplayRaw<'a> {
    display: St7305Display<'a>,
}
impl<'a> DisplayRaw<'a> {
    pub fn new<'b:'a>(io: DisplayIO<'b>) -> Result<Self> {
        log::info!("Initializing SPI driver...");
        let spi = SpiDriver::new(
            io.spi,
            io.sclk,
            io.mosi,
            Option::<AnyIOPin>::None, // MISO
            &SpiDriverConfig::new(),
        )?;
        log::info!("SPI driver initialized successfully");
        let device = SpiDeviceDriver::new(spi, Some(io.cs), &SpiConfig::new())?;
        log::info!("SPI device driver initialized successfully");
        let dc = PinDriver::output(io.dc)?; // DC
        let rst = PinDriver::output(io.rst)?; // RST
        log::info!("GPIO pins initialized successfully");
        let di = SPIInterface::new(device, dc);
        log::info!("SPI interface created successfully");
        let display = St7305::new(di, rst);
        log::info!("ST7305 display driver initialized successfully");
        Ok(DisplayRaw { display })
    }
    pub fn get_display(&self) -> &St7305Display<'a> {
        &self.display
    }
    pub fn get_display_mut(&mut self) -> &mut St7305Display<'a> {
        &mut self.display
    }
}

impl<'a> Debug for DisplayRaw<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DisplayRaw").finish()
    }
}
