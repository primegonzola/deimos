// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use anyhow::Result;
use std::fmt;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::KhrSwapchainExtension;
use winit::window::Window;

use crate::vulkan::{Texture, TextureView, VulkanApi, VulkanDevice};

#[repr(C)]
#[derive(Clone)]
pub struct Swapchain {
    pub handle: vk::SwapchainKHR,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    pub textures: Vec<Texture>,
    pub views: Vec<TextureView>,
}

impl Swapchain {
    #[inline]
    pub fn new(
        handle: vk::SwapchainKHR,
        format: vk::Format,
        extent: vk::Extent2D,
        textures: Vec<Texture>,
        views: Vec<TextureView>,
    ) -> Self {
        Self {
            handle,
            format,
            extent,
            textures,
            views,
        }
    }

    pub fn create(api: &VulkanApi, surface: vk::SurfaceKHR, window: &Window) -> Result<Swapchain> {
        // create swapchain
        let (swapchain, format, extent, images, views) = api.create_swapchain(window, surface)?;

        // save swapchain info
        Ok(Swapchain::new(
            swapchain,
            format,
            extent,
            images
                .iter()
                .map(|i| Texture::new(*i, vk::DeviceMemory::null(), 0))
                .collect(),
            views.iter().map(|i| TextureView::new(*i)).collect(),
        ))
    }

    #[inline]
    pub fn null() -> Self {
        Self {
            handle: vk::SwapchainKHR::null(),
            format: vk::Format::UNDEFINED,
            extent: vk::Extent2D::default(),
            textures: Vec::new(),
            views: Vec::new(),
        }
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self.handle == vk::SwapchainKHR::null()
    }

    #[inline]
    pub fn destroy(&self, device: &VulkanDevice) {
        unsafe {
            self.views.iter().for_each(|v| v.destroy(&device));

            // destroy swapchain
            device
                .api
                .logical_device
                .destroy_swapchain_khr(self.handle, None);
        }
    }
}

impl Default for Swapchain {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Debug for Swapchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Swapchain.handle({:p})",
            self.handle.as_raw() as *const u8
        )
    }
}
