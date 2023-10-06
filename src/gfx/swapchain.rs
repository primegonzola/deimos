#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::KhrSurfaceExtension;


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