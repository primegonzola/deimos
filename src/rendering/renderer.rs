#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use::anyhow::Result;

pub struct Renderer {


}

impl Renderer {
    pub fn create()->Result<Self> {
        Ok(Self{})
    }
}