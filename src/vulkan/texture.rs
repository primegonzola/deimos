// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use anyhow::Result;
use std::fmt;
use std::hash::Hash;
use vulkanalia::prelude::v1_0::*;

use crate::vulkan::{VulkanApi, VulkanDevice};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Texture {
    pub handle: vk::Image,
    pub memory: vk::DeviceMemory,
    pub mip_levels: u32,
}

impl Texture {
    #[inline]
    pub fn new(handle: vk::Image, memory: vk::DeviceMemory, mip_levels: u32) -> Self {
        Self {
            handle,
            memory,
            mip_levels,
        }
    }

    #[inline]
    pub fn null() -> Self {
        Self {
            handle: vk::Image::null(),
            memory: vk::DeviceMemory::null(),
            mip_levels: 0,
        }
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self.handle == vk::Image::null() && self.memory == vk::DeviceMemory::null()
    }

    #[inline]
    pub fn destroy(&self, device: &VulkanDevice) {
        unsafe {
            // destroy image
            device.api.logical_device.destroy_image(self.handle, None);
            // free memory
            device.api.logical_device.free_memory(self.memory, None);
        }
    }

    pub fn create(
        api: &VulkanApi,
        width: u32,
        height: u32,
        mip_levels: u32,
        samples: vk::SampleCountFlags,
        format: vk::Format,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<Texture> {
        // create image
        let (handle, memory) = api.create_image(
            width, height, mip_levels, samples, format, tiling, usage, properties,
        )?;
        // all done
        Ok(Texture::new(handle, memory, mip_levels))
    }

    pub fn load(
        api: &VulkanApi,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
        path: &str,
    ) -> Result<Texture> {
        let (image, memory, mip_levels) = api.load_image(queue, command_pool, path)?;
        Ok(Texture::new(image, memory, mip_levels))
    }

    pub fn create_view(
        &self,
        api: &VulkanApi,
        format: vk::Format,
        aspects: vk::ImageAspectFlags,
        mip_levels: u32,
    ) -> Result<TextureView> {
        Ok(TextureView::new(VulkanApi::create_image_view(
            &api.logical_device,
            self.handle,
            format,
            aspects,
            mip_levels,
        )?))
    }
}

impl Default for Texture {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Texture.handle({:p}) - Texture.memory({:p})",
            self.handle.as_raw() as *const u8,
            self.memory.as_raw() as *const u8
        )
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TextureView {
    pub handle: vk::ImageView,
}

impl TextureView {
    #[inline]
    pub fn new(handle: vk::ImageView) -> Self {
        Self { handle }
    }

    #[inline]
    pub fn null() -> Self {
        Self {
            handle: vk::ImageView::null(),
        }
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self.handle == vk::ImageView::null()
    }

    #[inline]
    pub fn destroy(&self, device: &VulkanDevice) {
        unsafe {
            // destroy image view
            device
                .api
                .logical_device
                .destroy_image_view(self.handle, None);
        }
    }
}

impl Default for TextureView {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for TextureView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ImageView.handle({:p})",
            self.handle.as_raw() as *const u8
        )
    }
}
