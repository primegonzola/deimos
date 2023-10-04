use anyhow::Result;
use cgmath::num_traits::ToPrimitive;

use super::device::Device;

pub struct Buffer<T> {
    pub handle: vulkano::buffer::Subbuffer<[T]>,
}

impl<T> Buffer<T> {
    pub fn from_iter<S>(
        device: &Device,
        usage: vulkano::buffer::BufferUsage,
        iter: S,
    ) -> Result<Buffer<T>>
    where
        T: vulkano::buffer::BufferContents,
        S: IntoIterator<Item = T>,
        S::IntoIter: ExactSizeIterator,
    {
        // create native
        let handle = vulkano::buffer::Buffer::from_iter(
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
        .unwrap();

        Ok(Self {
            handle: handle,
        })
    }

    pub fn len(&self) -> usize {
        // delegate to the handle
        self.handle.len().to_usize().unwrap()
    }
}
