#![allow(dead_code)]

mod display;
mod io;

pub use display::{DisplayRaw, St7305Display};
pub use io::DisplayIO;

pub const ESP32S3_LCP4_2_SCREEN_WIDTH: u32 = 400;
pub const ESP32S3_LCP4_2_SCREEN_HEIGHT: u32 = 300;
