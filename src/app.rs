// SPDX-License-Identifier: MIT

// #![allow(dead_code)]

use anyhow::Result;
use winit::window::Window;

use crate::graphics;

pub struct App {
    pub data: AppData,
    pub graphics: graphics::Device,
}

impl App {
    // Creates the app.
    pub fn create(window: &Window) -> Result<Self> {
        // init data
        let data = AppData::default();

        // create the graphics device
        let graphics = graphics::Device::create(window)?;

        // init app instance
        Ok(Self { graphics, data })
    }

    // update the app
    pub fn update(&mut self, window: &Window) -> Result<()> {
        // update the graphics device
        self.graphics.update(window)?;

        // all went fine
        Ok(())
    }

    // Destroys the app.
    pub fn destroy(&self) {
        // destroy graphics
        self.graphics.destroy();
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
