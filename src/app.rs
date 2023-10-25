// SPDX-License-Identifier: MIT

// #![allow(dead_code)]

use super::sample;
use anyhow::Result;
use winit::{dpi::PhysicalSize, window::Window};

/// the app.
pub struct App {
    pub data: AppData,
    pub sample: sample::Sample,
}

impl App {
    /// Creates the app.
    pub fn create(window: &Window) -> Result<Self> {
        // init data
        let data = AppData::default();

        // create sample
        let sample = sample::Sample::create(window)?;

        // init app instance
        Ok(Self { sample, data })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        // delegate to sample
        self.sample.resize(size);
    }

    /// update s a frame for the app.
    pub fn update(&mut self, window: &Window) -> Result<()> {
        // update
        self.sample.update(window)?;

        // all went fine
        Ok(())
    }

    /// Destroys the app.
    pub fn destroy(&mut self) {
        // destroy
        self.sample.destroy();
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
