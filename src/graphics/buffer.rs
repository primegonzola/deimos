#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::{anyhow, Result};
use std::fmt;
use std::hash::Hash;
use std::ptr::copy_nonoverlapping as memcpy;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::DeviceSize;

// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
}

impl Buffer {
    pub fn new(buffer: vk::Buffer, memory: vk::DeviceMemory) -> Self {
        Self { buffer, memory }
    }

    unsafe fn get_memory_type_index(
        instance: &Instance,
        physical: &vk::PhysicalDevice,
        properties: vk::MemoryPropertyFlags,
        requirements: vk::MemoryRequirements,
    ) -> Result<u32> {
        let memory = instance.get_physical_device_memory_properties(*physical);
        (0..memory.memory_type_count)
            .find(|i| {
                let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
                let memory_type = memory.memory_types[*i as usize];
                suitable && memory_type.property_flags.contains(properties)
            })
            .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
    }

    pub unsafe fn create(
        instance: &Instance,
        physical: &vk::PhysicalDevice,
        device: &Device,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<Buffer> {
        // create buffer info
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        // create native buffer
        let buffer = device.create_buffer(&buffer_info, None)?;

        // get memory requirements
        let requirements = device.get_buffer_memory_requirements(buffer);

        // get memory info
        let memory_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(Buffer::get_memory_type_index(
                instance,
                physical,
                properties,
                requirements,
            )?);

        // create memory
        let buffer_memory = device.allocate_memory(&memory_info, None)?;

        // bind memory
        device.bind_buffer_memory(buffer, buffer_memory, 0)?;

        // create instance
        Ok(Buffer::new(buffer, buffer_memory))
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the buffer
        device.destroy_buffer(self.buffer, None);

        // free memory
        device.free_memory(self.memory, None);
    }

    pub unsafe fn write<T>(
        &self,
        device: &Device,
        offset: DeviceSize,
        size: DeviceSize,
        data: &Vec<T>,
    ) {
        // lock memory
        let result = device.map_memory(self.memory, offset, size, vk::MemoryMapFlags::empty());

        // get data
        let memory = result.expect("Failed to map memory");

        // copy data into
        memcpy(data.as_ptr(), memory.cast(), data.len());

        // unlock memory
        device.unmap_memory(self.memory);
    }
}

impl Default for Buffer {
    #[inline]
    fn default() -> Self {
        Buffer::new(vk::Buffer::null(), vk::DeviceMemory::null())
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
