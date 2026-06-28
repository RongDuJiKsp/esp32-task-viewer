use std::sync::Mutex;

use anyhow::Result;
use battery_estimator::{BatteryChemistry, SocEstimator};

use crate::{adc::Adc, BatteryIO};

/// 电压分压比：实际电池电压 = ADC读取电压 × VOLTAGE_DIVIDER_RATIO
const VOLTAGE_DIVIDER_RATIO: f32 = 3.0;
/// ADC 参考电压 (mV)，Atten12dB 对应 ~3100mV
const ADC_REF_MV: f32 = 3100.0;
/// ESP32-S3 ADC 分辨率 (12-bit)
const ADC_MAX_RAW: f32 = 4095.0;

pub struct BatteryMonitor {
    adc: Mutex<Adc<'static>>,
    estimator: SocEstimator,
}

impl std::fmt::Debug for BatteryMonitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatteryMonitor")
            .field("adc", &self.adc)
            .finish_non_exhaustive()
    }
}

impl BatteryMonitor {
    pub fn new(io: BatteryIO<'static>, chemistry: BatteryChemistry) -> Result<Self> {
        let estimator = SocEstimator::new(chemistry);
        log::info!("Battery estimator created");

        Ok(Self { adc: Mutex::new(Adc::new(io)?), estimator })
    }

    /// 读取 ADC 原始值
    pub fn read_raw(&self) -> Result<u16> {
        self.adc.lock().map_err(|e| anyhow::anyhow!("{e}"))?.read_raw()
    }

    /// 读取电池电压 (mV)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn read_voltage_mv(&self) -> Result<u32> {
        let raw = f32::from(self.read_raw()?);
        let adc_mv = raw * ADC_REF_MV / ADC_MAX_RAW;
        let battery_mv = adc_mv * VOLTAGE_DIVIDER_RATIO;
        Ok(battery_mv as u32)
    }

    /// 读取电池电压 (V)
    #[allow(clippy::cast_precision_loss)]
    pub fn read_voltage_v(&self) -> Result<f32> {
        let mv = self.read_voltage_mv()?;
        Ok(mv as f32 / 1000.0)
    }

    /// 读取电池电量百分比 (0-100)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn read_soc(&self) -> Result<u8> {
        let voltage = self.read_voltage_v()?;
        let soc = self.estimator.estimate_soc(voltage).map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(soc as u8)
    }
}
