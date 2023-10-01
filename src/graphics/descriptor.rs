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
pub struct DescriptorPool {
    pub pool: vk::DescriptorPool,
}

impl DescriptorPool {
    pub fn create(pool: vk::DescriptorPool) -> Self {
        Self { pool }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy
        device.destroy_descriptor_pool(self.pool, None);
    }
}

impl Default for DescriptorPool {
    #[inline]
    fn default() -> Self {
        DescriptorPool::create(vk::DescriptorPool::null())
    }
}

impl fmt::Debug for DescriptorPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}



// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorSet {
    pub set: vk::DescriptorSet,
}

impl DescriptorSet {
    pub fn create(set: vk::DescriptorSet) -> Self {
        Self { set }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy
        // device.destroy_descriptor_set(self.set, None);
    }
}

impl Default for DescriptorSet {
    #[inline]
    fn default() -> Self {
        DescriptorSet::create(vk::DescriptorSet::null())
    }
}

impl fmt::Debug for DescriptorSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}