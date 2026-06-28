use embedded_graphics::pixelcolor::BinaryColor;
use slint::Rgb8Pixel;

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
        (u16::from(self.red) * 30 + u16::from(self.green) * 59 + u16::from(self.blue) * 11) / 100
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
