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
pub struct SwapChain {
    pub swapchain: vk::SwapchainKHR,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl SwapChain {
    pub fn new(swapchain: vk::SwapchainKHR, format: vk::Format, extent: vk::Extent2D) -> Self {
        Self { swapchain, format, extent }
    }

    pub unsafe fn destroy(&self, device: &Device) {}
}

impl Default for SwapChain {
    #[inline]
    fn default() -> Self {
        SwapChain::new(vk::SwapchainKHR::null(), vk::Format::default(), vk::Extent2D::default())
    }
}

impl fmt::Debug for SwapChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
