#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use std::fmt;
use std::hash::Hash;

use vulkanalia::prelude::v1_0::*;

// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RenderPass {
    pub pass: vk::RenderPass,
}

impl RenderPass {
    pub fn create(pass: vk::RenderPass) -> Self {
        Self { pass }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the render pass
        device.destroy_render_pass(self.pass, None);
    }
}

impl Default for RenderPass {
    #[inline]
    fn default() -> Self {
        RenderPass::create(vk::RenderPass::null())
    }
}

impl fmt::Debug for RenderPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
