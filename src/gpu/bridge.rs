use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use crate::gpu::{self};
use crate::vulkan::VulkanDevice;

#[derive(Clone)]
pub struct VulkanBridge {
    pub device: VulkanDevice,
}

impl VulkanBridge {
    pub fn create(device: VulkanDevice) -> Result<VulkanBridge> {
        Ok(Self { device })
    }

    pub fn construct_render_pass(
        &self,
        _descriptor: gpu::GPURenderPassDescriptor,
    ) -> Result<vk::RenderPass> {
        Ok(self
            .device
            .api
            .create_render_pass(self.device.data.swapchain.format, self.device.data.samples)?)
    }
}
