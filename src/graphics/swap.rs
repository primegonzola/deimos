#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use std::fmt;
use winit::window::Window;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;

use super::QueueFamilyIndices;

// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SwapChain {
    pub swapchain: vk::SwapchainKHR,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl SwapChain {
    pub fn new(swapchain: vk::SwapchainKHR, format: vk::Format, extent: vk::Extent2D) -> Self {
        Self {
            swapchain,
            format,
            extent,
        }
    }

    fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        formats
            .iter()
            .cloned()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or_else(|| formats[0])
    }

    fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        present_modes
            .iter()
            .cloned()
            .find(|m| *m == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    fn get_swapchain_extent(
        window: &Window,
        capabilities: vk::SurfaceCapabilitiesKHR,
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::max_value() {
            capabilities.current_extent
        } else {
            let size = window.inner_size();
            let clamp = |min: u32, max: u32, v: u32| min.max(max.min(v));
            vk::Extent2D::builder()
                .width(clamp(
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                    size.width,
                ))
                .height(clamp(
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                    size.height,
                ))
                .build()
        }
    }

    pub unsafe fn create(
        window: &Window,
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        physical: &vk::PhysicalDevice,
        device: &Device,
    ) -> Result<SwapChain> {
        let indices = QueueFamilyIndices::get(instance, surface, *physical)?;
        let support = SwapChainSupport::get(instance, surface, *physical)?;

        let surface_format = SwapChain::get_swapchain_surface_format(&support.formats);
        let present_mode = SwapChain::get_swapchain_present_mode(&support.present_modes);
        let extent = SwapChain::get_swapchain_extent(window, support.capabilities);

        let format = surface_format.format;
        let extent = extent;

        let mut image_count = support.capabilities.min_image_count + 1;
        if support.capabilities.max_image_count != 0
            && image_count > support.capabilities.max_image_count
        {
            image_count = support.capabilities.max_image_count;
        }

        let mut queue_family_indices = vec![];
        let image_sharing_mode = if indices.graphics != indices.present {
            queue_family_indices.push(indices.graphics);
            queue_family_indices.push(indices.present);
            vk::SharingMode::CONCURRENT
        } else {
            vk::SharingMode::EXCLUSIVE
        };

        // build info
        let info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        // create swap chain
        let swapchain = device.create_swapchain_khr(&info, None)?;

        // all went fine
        Ok(SwapChain::new(swapchain, format, extent))
    }
    
    pub unsafe fn destroy(&self, device: &Device) {}
}

impl Default for SwapChain {
    #[inline]
    fn default() -> Self {
        SwapChain::new(
            vk::SwapchainKHR::null(),
            vk::Format::default(),
            vk::Extent2D::default(),
        )
    }
}

impl fmt::Debug for SwapChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SwapChainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapChainSupport {
    pub unsafe fn get(
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        Ok(Self {
            capabilities: instance
                .get_physical_device_surface_capabilities_khr(physical_device, *surface)?,
            formats: instance.get_physical_device_surface_formats_khr(physical_device, *surface)?,
            present_modes: instance
                .get_physical_device_surface_present_modes_khr(physical_device, *surface)?,
        })
    }
}
