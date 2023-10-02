use anyhow::Result;
use std::sync::Arc;

use super::device::Device;

pub struct Buffer {
    _handle: Arc<vulkano::buffer::Buffer>,
}

impl Buffer {
    pub fn from_iter<T, I>(
        device: &Device,
        usage: vulkano::buffer::BufferUsage,
        iter: I,
    ) -> Result<vulkano::buffer::Subbuffer<[T]>>
    where
        T: vulkano::buffer::BufferContents,
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Ok(vulkano::buffer::Buffer::from_iter(
            &device.memory_allocator,
            vulkano::buffer::BufferCreateInfo {
                usage: usage,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo {
                usage: vulkano::memory::allocator::MemoryUsage::Upload,
                ..Default::default()
            },
            iter,
        )
        .unwrap())
    }
}
