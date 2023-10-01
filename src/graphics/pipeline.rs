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
pub struct Pipeline {
    pub pipeline: vk::Pipeline,
}

impl Pipeline {
    pub fn create(pipeline: vk::Pipeline) -> Self {
        Self { pipeline }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the sampler
        // device.destroy_sampler(self.sampler, None);
    }
}

impl Default for Pipeline {
    #[inline]
    fn default() -> Self {
        Pipeline::create(vk::Pipeline::null())
    }
}

impl fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
