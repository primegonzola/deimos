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
pub struct Queue {
    pub queue: vk::Queue,
}

impl Queue {
    pub fn create(queue: vk::Queue) -> Self {
        Self { queue }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the queue
        // device.destroy_(self.queue, None);
    }
}

impl Default for Queue {
    #[inline]
    fn default() -> Self {
        Queue::create(vk::Queue::null())
    }
}

impl fmt::Debug for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
