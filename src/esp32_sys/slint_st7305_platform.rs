use core::cell::RefCell;
use std::rc::Rc;

use slint::platform::{
    software_renderer::{LineBufferProvider, MinimalSoftwareWindow, Rgb565Pixel},
    Platform,
};

use super::display_raw::St7305Display;
pub struct SlintSt7305Platform<'a> {
    window: Rc<MinimalSoftwareWindow>,
    display: RefCell<St7305Display<'a>>,
}
impl<'a> Platform for SlintSt7305Platform<'a> {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }
}
impl<'a> LineBufferProvider for SlintSt7305Platform<'a> {
    type TargetPixel = Rgb565Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: std::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        todo!()
    }
}
