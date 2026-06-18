use super::lib::display_raw::{
    DisplayRaw, ESP32S3_LCP4_2_SCREEN_HEIGHT, ESP32S3_LCP4_2_SCREEN_WIDTH,
};
use anyhow::Result;
use core::time::Duration;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Point, Pixel};
use slint::{
    platform::{
        software_renderer::{
            LineBufferProvider, MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel,
        },
        Platform, WindowAdapter,
    },
    PhysicalSize, PlatformError, Rgb8Pixel,
};

use std::{
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};

pub struct BlackPixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl BlackPixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        BlackPixel { red, green, blue }
    }
    pub fn get_gray(&self) -> u16 {
        (self.red as u16 * 30 + self.green as u16 * 59 + self.blue as u16 * 11) / 100
    }
    pub fn is_black(&self) -> bool {
        self.get_gray() > 128
    }
}

impl From<Rgb8Pixel> for BlackPixel {
    fn from(val: Rgb8Pixel) -> Self {
        BlackPixel::new(val.r, val.g, val.b)
    }
}

impl From<BlackPixel> for Rgb8Pixel {
    fn from(val: BlackPixel) -> Self {
        Rgb8Pixel::new(val.red, val.green, val.blue)
    }
}

impl From<BlackPixel> for BinaryColor {
    fn from(val: BlackPixel) -> Self {
        if val.is_black() {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }
}

pub struct SlintSt7305Platform {
    window: Rc<MinimalSoftwareWindow>,
    platform_display: SlintSt7305PlatformDisplay,
}
impl SlintSt7305Platform {
    pub fn new(display: Arc<DisplayRaw>) -> Self {
        let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
        window.set_size(PhysicalSize::new(
            ESP32S3_LCP4_2_SCREEN_WIDTH,
            ESP32S3_LCP4_2_SCREEN_HEIGHT,
        ));
        Self {
            window,
            platform_display: SlintSt7305PlatformDisplay::new(display),
        }
    }
}
impl Platform for SlintSt7305Platform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> Duration {
        Duration::from_millis(unsafe { esp_idf_hal::sys::esp_timer_get_time() as u64 } / 1000)
    }
    fn run_event_loop(&self) -> Result<(), PlatformError> {
        loop {
            self.window.draw_if_needed(|renderer| {
                renderer.render_by_line(&self.platform_display);
            });

            self.platform_display
                .get_display_raw()
                .get_display()
                .map_err(|e| PlatformError::from(format!("{e}")))?
                .flush()
                .map_err(|e| PlatformError::from(format!("{:#?}", e)))?;

            esp_idf_hal::delay::FreeRtos::delay_ms(16);
        }
    }
}

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
        self.line_buffer
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock line buffer: {:#?}", e))
    }
}

impl LineBufferProvider for &SlintSt7305PlatformDisplay {
    type TargetPixel = Rgb565Pixel;
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
