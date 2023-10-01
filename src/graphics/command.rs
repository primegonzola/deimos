#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use std::fmt;
use std::hash::Hash;

use vulkanalia::prelude::v1_0::*;

use super::Queue;

// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CommandBuffer {
    pub buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    pub fn new(buffer: vk::CommandBuffer) -> Self {
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
        CommandBuffer::new(vk::CommandBuffer::null())
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
    pub fn new(pool: vk::CommandPool) -> Self {
        Self { pool }
    }

    pub unsafe fn begin_single(device: &Device, pool: &CommandPool) -> Result<CommandBuffer> {
        // create the info for allocating the command buffer
        let info = vk::CommandBufferAllocateInfo::builder()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(pool.pool)
            .command_buffer_count(1);

        // allocate the buffer
        let cb = device.allocate_command_buffers(&info)?[0];

        // create the command buffer info
        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        // begin the command buffer
        device.begin_command_buffer(cb, &info)?;

        // all done
        Ok(CommandBuffer::new(cb))
    }

    pub unsafe fn end_single(
        device: &Device,
        pool: &CommandPool,
        queue: &Queue,
        command_buffer: CommandBuffer,
    ) -> Result<()> {
        device.end_command_buffer(command_buffer.buffer)?;

        // submit the command buffer
        let command_buffers = &[command_buffer.buffer];

        // create the info for submitting the command buffer
        let info = vk::SubmitInfo::builder().command_buffers(command_buffers);

        // add to queue
        device.queue_submit(queue.queue, &[info], vk::Fence::null())?;

        // wait queue to be idle
        device.queue_wait_idle(queue.queue)?;

        // cleanup
        device.free_command_buffers(pool.pool, &[command_buffer.buffer]);

        // all ok
        Ok(())
    }
    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the pool
        device.destroy_command_pool(self.pool, None);
    }
}

impl Default for CommandPool {
    #[inline]
    fn default() -> Self {
        CommandPool::new(vk::CommandPool::null())
    }
}

impl fmt::Debug for CommandPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
