#![allow(dead_code)]
use anyhow::Result;
use core::fmt::Debug;
use display_interface_spi::SPIInterface;
use esp_idf_hal::{
    gpio::{AnyIOPin, PinDriver},
    spi::{SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use std::sync::Mutex;
use st7305::{BinaryColor, Orientation, St7305};

pub type St7305Display<'a> = St7305<
    SPIInterface<SpiDeviceDriver<'a, SpiDriver<'a>>, PinDriver<'a, esp_idf_hal::gpio::Output>>,
    PinDriver<'a, esp_idf_hal::gpio::Output>,
>;

pub const ESP32S3_LCP4_2_SCREEN_WIDTH: u32 = 400;
pub const ESP32S3_LCP4_2_SCREEN_HEIGHT: u32 = 300;

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

pub struct DisplayRaw {
    display: Mutex<St7305Display<'static>>,
}

impl DisplayRaw {
    pub fn new(io: DisplayIO<'static>) -> Result<Self> {
        let spi = SpiDriver::new(
            io.spi,
            io.sclk,
            io.mosi,
            Option::<AnyIOPin>::None, // MISO
            &SpiDriverConfig::new(),
        )?;
        let device = SpiDeviceDriver::new(spi, Some(io.cs), &SpiConfig::new())?;
        let dc = PinDriver::output(io.dc)?; // DC
        let rst = PinDriver::output(io.rst)?; // RST
        let di = SPIInterface::new(device, dc);
        let display = Mutex::new(St7305::new(di, rst));
        log::info!("ST7305 display driver initialized successfully");
        Ok(DisplayRaw { display })
    }

    pub fn init(&self) -> Result<()> {
        let mut delay = esp_idf_hal::delay::Ets;
        let mut display = self.get_display()?;
        display
            .init(&mut delay)
            .map_err(|e| anyhow::anyhow!("Failed to initialize display: {:#?}", e))?;
        display.set_orientation(Orientation::Landscape);
        display.color_clear(BinaryColor::Off as u8);
        display
            .flush()
            .map_err(|e| anyhow::anyhow!("Failed to initialize display: {:#?}", e))?;
        Ok(())
    }

    pub fn get_display(&self) -> Result<std::sync::MutexGuard<'_, St7305Display<'static>>> {
        self.display
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock display: {:#?}", e))
    }
}

impl Debug for DisplayRaw {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DisplayRaw").finish()
    }
}
