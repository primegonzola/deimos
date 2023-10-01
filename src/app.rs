// SPDX-License-Identifier: MIT

#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::window::Window;

use crate::graphics::GraphicsDevice;

/// the app.
#[derive(Clone, Debug)]
pub struct App {
    pub graphics: GraphicsDevice,
    pub data: AppData,
}

impl App {
    /// Creates the app.
    pub unsafe fn create(window: &Window) -> Result<Self> {
        // init data
        let data = AppData::default();

        // create graphics device
        let graphics = GraphicsDevice::create(window)?;

        // init app instance
        Ok(Self { graphics, data })
    }

    /// update s a frame for the app.
    pub unsafe fn update(&mut self, window: &Window) -> Result<()> {
        // update graphics device
        self.graphics.update(window, self.data.models)?;

        // all went fine
        Ok(())
    }

    /// Destroys the app.
    #[rustfmt::skip]
    pub unsafe fn destroy(&mut self) {

        // destroy the graphics device
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
