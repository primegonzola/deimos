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

use vulkanalia::bytecode::Bytecode;
use vulkanalia::prelude::v1_0::*;

// #[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Shader {
    pub module: vk::ShaderModule,
}

impl Shader {
    pub fn new(module: vk::ShaderModule) -> Self {
        Self { module }
    }

    pub unsafe fn create(device: &Device, code: &[u8]) -> Result<Shader> {
        // get bytes
        let bytes = Bytecode::new(code).unwrap();

        // create info
        let info = vk::ShaderModuleCreateInfo::builder()
            .code_size(bytes.code_size())
            .code(bytes.code());

        // create new shader
        Ok(Shader::new(device.create_shader_module(&info, None)?))
    }

    pub unsafe fn destroy(&self, device: &Device) {
        // destroy the shader
        device.destroy_shader_module(self.module, None);
    }
}

impl Default for Shader {
    #[inline]
    fn default() -> Self {
        Shader::new(vk::ShaderModule::null())
    }
}

impl fmt::Debug for Shader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Memory({:p})", self.0 as *const u8)
        // write!(f, "Image({:p}) - Memory({:p})", self.0 as *const u8, self.0 as *const u8)
        Ok(())
    }
}
