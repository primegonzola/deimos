#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use std::fmt;
use std::hash::Hash;

use super::super::graphics::Buffer;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Mesh {
    vertices: Buffer,
    indices: Buffer,
}

impl Mesh {
    pub fn create(vertices:Buffer, indices:Buffer) -> Self {
        Self { vertices, indices }
    }
}

// impl Default for Sampler {
//     #[inline]
//     fn default() -> Self {
//         Sampler::create(vk::Sampler::null())
//     }
// }

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
