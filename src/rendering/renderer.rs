// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use anyhow::Result;
use std::sync::Arc;

use super::super::graphics::Device;

pub struct Renderer {
    graphics: Arc<Device>,
}

impl Renderer {
    pub fn create(graphics: Arc<Device>) -> Result<Self> {
        Ok(Self {
            graphics: graphics.clone(),
        })
    }
}
