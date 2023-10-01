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
pub struct FrameBuffer {
    pub buffer: vk::Framebuffer,
}

impl FrameBuffer {
    pub fn create(buffer: vk::Framebuffer) -> Self {
        Self { buffer }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the buffer
        device.destroy_framebuffer(self.buffer, None);
    }
}

impl Default for FrameBuffer {
    #[inline]
    fn default() -> Self {
        FrameBuffer::create(vk::Framebuffer::null())
    }
}

impl fmt::Debug for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
