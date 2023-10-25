// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use std::hash::{Hash, Hasher};
use std::mem::size_of;
use vulkanalia::prelude::v1_0::*;

pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BasicVertex {
    pub position: Vec3,
    pub texel: Vec2,
}

impl BasicVertex {
    fn new(position: Vec3, texel: Vec2) -> Self {
        Self { position, texel }
    }
}

impl PartialEq for BasicVertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.texel == other.texel
    }
}

impl Eq for BasicVertex {}

impl Hash for BasicVertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position[0].to_bits().hash(state);
        self.position[1].to_bits().hash(state);
        self.position[2].to_bits().hash(state);
        self.texel[0].to_bits().hash(state);
        self.texel[1].to_bits().hash(state);
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Mesh {}

impl Mesh {
    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<BasicVertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let position = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
            .build();
        let texel = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32_SFLOAT)
            .offset((size_of::<cgmath::Vector3<f32>>()) as u32)
            .build();
        [position, texel]
    }
}

