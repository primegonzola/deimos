#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use std::fmt;
use std::hash::Hash;

use vulkanalia::prelude::v1_0::*;

use super::RenderPass;
use super::TextureView;

// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct FrameBuffer {
    pub buffer: vk::Framebuffer,
}

impl FrameBuffer {
    pub fn new(buffer: vk::Framebuffer) -> Self {
        Self { buffer }
    }

    pub unsafe fn create(
        device: &Device,
        pass: &RenderPass,
        attachments: &[TextureView],
        width: u32,
        height: u32,
    ) -> Result<FrameBuffer> {
        // get raw vies
        let views = attachments.iter().map(|v| v.view).collect::<Vec<_>>();

        // info to use for creating the buffer
        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(pass.pass)
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        let buffer = FrameBuffer::new(
            device
                .create_framebuffer(&info, None)
                .expect("Failed to create framebuffer."),
        );

        // all done
        Ok(buffer)
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the buffer
        device.destroy_framebuffer(self.buffer, None);
    }
}

impl Default for FrameBuffer {
    #[inline]
    fn default() -> Self {
        FrameBuffer::new(vk::Framebuffer::null())
    }
}

impl fmt::Debug for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
