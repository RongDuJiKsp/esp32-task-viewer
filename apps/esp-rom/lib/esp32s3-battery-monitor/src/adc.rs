use anyhow::Result;
use esp_idf_hal::adc::{
    attenuation,
    oneshot::{
        config::{AdcChannelConfig, Calibration},
        AdcChannelDriver, AdcDriver,
    },
    Resolution, ADCCH3, ADCU1,
};

use crate::BatteryIO;

pub struct Adc<'a> {
    channel: AdcChannelDriver<'a, ADCCH3<ADCU1>, AdcDriver<'a, ADCU1>>,
}

impl std::fmt::Debug for Adc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Adc").finish_non_exhaustive()
    }
}

impl<'a> Adc<'a> {
    pub fn new(io: BatteryIO<'a>) -> Result<Self> {
        log::info!("Initializing ADC...");
        let driver = AdcDriver::new(io.adc)?;
        let driver_cfg = AdcChannelConfig {
            attenuation: attenuation::DB_12,
            resolution: Resolution::Resolution12Bit,
            calibration: Calibration::Curve,
        };
        let channel = AdcChannelDriver::new(driver, io.battery_pin, &driver_cfg)?;
        log::info!("ADC initialized successfully");
        Ok(Self { channel })
    }

    pub fn read_raw(&mut self) -> Result<u16> {
        self.channel.read_raw().map_err(|e| anyhow::anyhow!("{e}"))
    }
}
