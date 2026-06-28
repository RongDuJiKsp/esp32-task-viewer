use core::time::Duration;
use std::{rc::Rc, sync::Arc};

use anyhow::Result;
use esp32s3_st7305_lcd_display::{
    DisplayRaw, ESP32S3_LCP4_2_SCREEN_HEIGHT, ESP32S3_LCP4_2_SCREEN_WIDTH,
};
use slint::{
    platform::{software_renderer::MinimalSoftwareWindow, Platform, WindowAdapter},
    PhysicalSize, PlatformError,
};

use crate::display::SlintSt7305PlatformDisplay;

pub struct SlintSt7305Platform {
    window: Rc<MinimalSoftwareWindow>,
    platform_display: SlintSt7305PlatformDisplay,
}

type EventLoopResult = Result<(), PlatformError>;

impl SlintSt7305Platform {
    pub fn new(display: Arc<DisplayRaw>) -> Self {
        let window = MinimalSoftwareWindow::new(
            slint::platform::software_renderer::RepaintBufferType::ReusedBuffer,
        );
        window
            .set_size(PhysicalSize::new(ESP32S3_LCP4_2_SCREEN_WIDTH, ESP32S3_LCP4_2_SCREEN_HEIGHT));
        Self { window, platform_display: SlintSt7305PlatformDisplay::new(display) }
    }

    pub fn event_loop_timer(&self) -> EventLoopResult {
        slint::platform::update_timers_and_animations();
        Ok(())
    }

    pub fn event_loop_render(&self) -> EventLoopResult {
        let rendered = self.window.draw_if_needed(|renderer| {
            let _reg = renderer.render_by_line(&self.platform_display);
        });

        if rendered {
            self.platform_display
                .get_display_raw()
                .get_display()
                .map_err(|e| PlatformError::from(format!("{e}")))?
                .flush()
                .map_err(|e| PlatformError::from(format!("{:#?}", e)))?;
        }
        Ok(())
    }

    pub fn event_loop_wait(&self) -> EventLoopResult {
        if self.window.has_active_animations() {
            esp_idf_hal::delay::FreeRtos::delay_ms(16);
        } else {
            esp_idf_hal::delay::FreeRtos::delay_ms(200);
        }
        Ok(())
    }
}

impl Platform for SlintSt7305Platform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn duration_since_start(&self) -> Duration {
        Duration::from_millis(
            unsafe { esp_idf_hal::sys::esp_timer_get_time().cast_unsigned() } / 1000,
        )
    }

    fn run_event_loop(&self) -> Result<(), PlatformError> {
        loop {
            self.event_loop_timer()?;
            self.event_loop_render()?;
            self.event_loop_wait()?;
        }
    }
}
