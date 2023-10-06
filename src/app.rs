// SPDX-License-Identifier: MIT

#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use crate::gfx;
use anyhow::Result;
use winit::window::Window;

/// the app.
pub struct App {
    pub graphics: gfx::Device,
    pub data: AppData,
}

impl App {
    /// Creates the app.
    pub unsafe fn create(window: &Window) -> Result<Self> {
        // create graphics
        let graphics = gfx::Device::create(window, "D E I M O S")?;

        // init data
        let data = AppData::default();

        // init app instance
        Ok(Self { graphics, data })
    }

    /// update s a frame for the app.
    pub unsafe fn update(&mut self, window: &Window) -> Result<()> {
        // all went fine
        Ok(())
    }

    /// Destroys the app.
    #[rustfmt::skip]
    pub unsafe fn destroy(&self) {  
        // destroy graphics
        self.graphics.destroy();
    }
}

/// The api handles and associated properties used by the app.
#[derive(Clone, Debug)]
pub struct AppData {
    pub models: usize,
}

impl Default for AppData {
    fn default() -> Self {
        AppData { models: 1 }
    }
}
