// SPDX-License-Identifier: MIT

#![allow(
    dead_code,
)]

use anyhow::Result;
use winit::window::Window;

/// the app.
#[derive(Clone, Debug)]
pub struct App {
    pub data: AppData,
}

impl App {
    /// Creates the app.
    pub unsafe fn create(_window: &Window) -> Result<Self> {
        // init data
        let data = AppData::default();

        // init app instance
        Ok(Self { data })
    }

    /// update s a frame for the app.
    pub unsafe fn update(&mut self, _window: &Window) -> Result<()> {
        // all went fine
        Ok(())
    }

    /// Destroys the app.
    pub unsafe fn destroy(&self) {
        // destroy any app data
    }
}

/// The api handles and associated properties used by the app.
#[derive(Clone, Debug)]
pub struct AppData {
    pub counter: u32,
}

impl Default for AppData {
    fn default() -> Self {
        AppData { counter: 0 }
    }
}
