use core::fmt::Debug;
use std::sync::Mutex;

use anyhow::Result;
use display_interface_spi::SPIInterface;
use esp_idf_hal::{
    gpio::{AnyIOPin, PinDriver,Output},
    spi::{SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use st7305::{BinaryColor, Orientation, St7305};

use crate::DisplayIO;

pub type St7305Display<'a> = St7305<
    SPIInterface<SpiDeviceDriver<'a, SpiDriver<'a>>, PinDriver<'a, Output>>,
    PinDriver<'a, Output>,
>;

pub struct DisplayRaw {
    display: Mutex<St7305Display<'static>>,
}

impl DisplayRaw {
    pub fn new(io: DisplayIO<'static>) -> Result<Self> {
        let spi = SpiDriver::new(
            io.spi,
            io.sclk,
            io.mosi,
            Option::<AnyIOPin<'_>>::None, // MISO
            &SpiDriverConfig::new(),
        )?;
        let device = SpiDeviceDriver::new(spi, Some(io.cs), &SpiConfig::new())?;
        let dc = PinDriver::output(io.dc)?; // DC
        let rst = PinDriver::output(io.rst)?; // RST
        let di = SPIInterface::new(device, dc);
        let display = Mutex::new(St7305::new(di, rst));
        log::info!("ST7305 display initialized successfully");
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
        display.flush().map_err(|e| anyhow::anyhow!("Failed to flush display: {:#?}", e))?;
        Ok(())
    }

    pub fn get_display(&self) -> Result<std::sync::MutexGuard<'_, St7305Display<'static>>> {
        self.display.lock().map_err(|e| anyhow::anyhow!("Failed to lock display: {:#?}", e))
    }
}

impl Debug for DisplayRaw {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DisplayRaw").finish()
    }
}
