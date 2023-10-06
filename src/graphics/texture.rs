#![allow(dead_code)]

use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

pub struct Texture {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
}

impl Texture {
    pub fn create(image: vk::Image, memory: vk::DeviceMemory) -> Self {
        Self { image, memory }
    }

    pub unsafe fn create_view(
        &self,
        device: &Device,
        format: vk::Format,
        aspects: vk::ImageAspectFlags,
        mip_levels: u32,
    ) -> Result<TextureView> {
        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspects)
            .base_mip_level(0)
            .level_count(mip_levels)
            .base_array_layer(0)
            .layer_count(1);

        let info = vk::ImageViewCreateInfo::builder()
            .image(self.image)
            .view_type(vk::ImageViewType::_2D)
            .format(format)
            .subresource_range(subresource_range);

        Ok(TextureView::create(device.create_image_view(&info, None)?))
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // check if memory has been allocated
        if self.memory != vk::DeviceMemory::null() {
            // destroy the image
            device.destroy_image(self.image, None);

            // free the memory
            device.free_memory(self.memory, None);
        }
    }
}

pub struct TextureView {
    pub view: vk::ImageView,
}

impl TextureView {
    pub fn create(view: vk::ImageView) -> Self {
        Self { view }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the image view
        device.destroy_image_view(self.view, None);
    }
}
