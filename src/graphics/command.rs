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
pub struct CommandBuffer {
    pub buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    pub fn create(buffer: vk::CommandBuffer) -> Self {
        Self { buffer }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the buffer
        // device.destroy_com(self.buffer, None);
    }
}

impl Default for CommandBuffer {
    #[inline]
    fn default() -> Self {
        CommandBuffer::create(vk::CommandBuffer::null())
    }
}

impl fmt::Debug for CommandBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}

// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CommandPool {
    pub pool: vk::CommandPool,
}

impl CommandPool {
    pub fn create(pool: vk::CommandPool) -> Self {
        Self { pool }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the pool
        device.destroy_command_pool(self.pool, None);
    }
}

impl Default for CommandPool {
    #[inline]
    fn default() -> Self {
        CommandPool::create(vk::CommandPool::null())
    }
}

impl fmt::Debug for CommandPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
