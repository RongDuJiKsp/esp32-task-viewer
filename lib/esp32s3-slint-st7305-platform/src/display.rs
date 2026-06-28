use std::sync::{Arc, Mutex, MutexGuard};

use anyhow::Result;
use embedded_graphics::{draw_target::DrawTarget, prelude::Point, Pixel};
use esp32s3_st7305_lcd_display::{DisplayRaw, ESP32S3_LCP4_2_SCREEN_WIDTH};
use slint::{
    platform::software_renderer::{LineBufferProvider, Rgb565Pixel},
    Rgb8Pixel,
};

use crate::pixel::BlackPixel;

pub struct SlintSt7305PlatformDisplay {
    display: Arc<DisplayRaw>,
    line_buffer: Mutex<[Rgb565Pixel; ESP32S3_LCP4_2_SCREEN_WIDTH as usize]>,
}

impl SlintSt7305PlatformDisplay {
    pub fn new(display: Arc<DisplayRaw>) -> Self {
        Self {
            display,
            line_buffer: Mutex::new([Rgb565Pixel::default(); ESP32S3_LCP4_2_SCREEN_WIDTH as usize]),
        }
    }

    pub fn get_display_raw(&self) -> &DisplayRaw {
        &self.display
    }

    pub fn get_buffer(
        &self,
    ) -> Result<MutexGuard<'_, [Rgb565Pixel; ESP32S3_LCP4_2_SCREEN_WIDTH as usize]>> {
        self.line_buffer.lock().map_err(|e| anyhow::anyhow!("Failed to lock line buffer: {:#?}", e))
    }
}

impl LineBufferProvider for &SlintSt7305PlatformDisplay {
    type TargetPixel = Rgb565Pixel;
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn process_line(
        &mut self,
        y: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        if range.len() > ESP32S3_LCP4_2_SCREEN_WIDTH as usize {
            log::warn!("Range length exceeds buffer size: {}", range.len());
        }
        let mut line_buffer = self.get_buffer().unwrap();
        let pixels = &mut line_buffer[0..range.len().min(ESP32S3_LCP4_2_SCREEN_WIDTH as usize)];
        render_fn(pixels);
        let mut display = self.display.get_display().unwrap();

        display.draw_iter(pixels.iter().enumerate().map(|(i, px)| {
            let color = Rgb8Pixel::from(*px);
            let x = range.start + i;
            let black_pixel: BlackPixel = color.into();
            Pixel(Point::new(x as i32, y as i32), black_pixel.into())
        }));
    }
}
