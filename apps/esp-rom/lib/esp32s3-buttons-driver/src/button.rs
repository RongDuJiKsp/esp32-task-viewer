use anyhow::Result;
use esp_idf_hal::gpio::{Input, PinDriver, Pull};

use crate::ButtonsIO;
pub struct Button<'a> {
    boot: PinDriver<'a, Input>,
    key: PinDriver<'a, Input>,
}
impl<'a> Button<'a> {
    pub fn new(io: ButtonsIO<'static>) -> Result<Self> {
        let boot = PinDriver::input(io.boot, Pull::Up)?;
        let key = PinDriver::input(io.key, Pull::Up)?;
        Ok(Self { boot, key })
    }

    pub fn is_boot_pressed(&self) -> bool {
        self.boot.is_low()
    }

    pub fn is_key_pressed(&self) -> bool {
        self.key.is_low()
    }
}
