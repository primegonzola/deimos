#![allow(dead_code, unused_variables)]

use super::TextureView;
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub struct FrameBuffer {
    pub buffer: vk::Framebuffer,
}

impl FrameBuffer {
    pub unsafe fn create(
        device: &vulkanalia::Device,
        pass: &vk::RenderPass,
        attachments: &[TextureView],
        width: u32,
        height: u32,
    ) -> Result<FrameBuffer> {
        // get raw vies
        let views = attachments.iter().map(|v| v.view).collect::<Vec<_>>();

        // info to use for creating the buffer
        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(*pass)
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);

        let buffer = device
            .create_framebuffer(&info, None)
            .expect("Failed to create framebuffer.");

        // all done
        Ok(FrameBuffer { buffer })
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the buffer
        device.destroy_framebuffer(self.buffer, None);
    }
}
