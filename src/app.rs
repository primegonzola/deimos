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

/// the app.
#[derive(Clone, Debug)]
pub struct App {
    pub data: AppData,
}

impl App {
    /// Creates the app.
    pub unsafe fn create(window: &Window) -> Result<Self> {
        // init data
        let data = AppData::default();

        // init app instance
        Ok(Self { data })
    }

    /// update s a frame for the app.
    pub unsafe fn update(&mut self, window: &Window) -> Result<()> {
        // all went fine
        Ok(())
    }

    /// Destroys the app.
    #[rustfmt::skip]
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
