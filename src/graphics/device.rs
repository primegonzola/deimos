// SPDX-License-Identifier: MIT

// #![allow(
//     dead_code,
// )]

use anyhow::Result;
use winit::window::Window;

pub struct Device {}

impl Device {
    pub fn create(_window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    pub fn update(&self, _window: &Window) -> Result<()> {
        Ok(())
    }

    pub fn destroy(&self) {}
}
