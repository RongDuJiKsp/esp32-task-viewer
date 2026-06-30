use core::fmt::Debug;
use std::sync::{Mutex, MutexGuard};

use anyhow::Result;

use crate::{Button, ButtonsIO};

pub struct ButtonViewer {
    inner: Mutex<Button<'static>>,
}

impl ButtonViewer {
    pub fn new(io: ButtonsIO<'static>) -> Result<Self> {
        Ok(Self { inner: Mutex::new(Button::new(io)?) })
    }

    pub fn button_raw(&self) -> Result<MutexGuard<'_, Button<'static>>> {
        self.inner.lock().map_err(|e| anyhow::anyhow!("{e}"))
    }
}

impl Debug for ButtonViewer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ButtonViewer").finish()
    }
}
