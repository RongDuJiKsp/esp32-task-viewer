use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Point, Pixel};
use std::rc::Rc;
use std::sync::Arc;

use slint::{
    platform::{
        software_renderer::{
            LineBufferProvider, MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel,
        },
        Platform, WindowAdapter,
    },
    PlatformError, Rgb8Pixel,
};

use super::lib::display_raw::DisplayRaw;

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

impl Into<BlackPixel> for Rgb8Pixel {
    fn into(self) -> BlackPixel {
        BlackPixel::new(self.r, self.g, self.b)
    }
}

impl Into<Rgb8Pixel> for BlackPixel {
    fn into(self) -> Rgb8Pixel {
        Rgb8Pixel::new(self.red, self.green, self.blue)
    }
}

impl Into<BinaryColor> for BlackPixel {
    fn into(self) -> BinaryColor {
        if self.is_black() {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }
}

pub struct SlintSt7305Platform {
    window: Rc<MinimalSoftwareWindow>,
    display: Arc<DisplayRaw>,
    line_buffer: [Rgb565Pixel; 400],
}
impl SlintSt7305Platform {
    pub fn new(display: Arc<DisplayRaw>) -> Self {
        Self {
            window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
            display,
            line_buffer: [Rgb565Pixel::default(); 400],
        }
    }
}
impl Platform for SlintSt7305Platform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}
impl LineBufferProvider for SlintSt7305Platform {
    type TargetPixel = Rgb565Pixel;
    fn process_line(
        &mut self,
        y: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        if range.len() > 400 {
            log::warn!("Range length exceeds buffer size: {}", range.len());
        }

        let pixels = &mut self.line_buffer[0..range.len().min(400)];
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
