// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use anyhow::Result;
use cgmath::{vec2, vec3};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::mem::size_of;
use vulkanalia::prelude::v1_0::*;

use crate::gpu::{self};

use super::GPUBufferUsageFlags;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BasicVertex {
    pub position: cgmath::Vector3<f32>,
    pub texel: cgmath::Vector2<f32>,
}

impl BasicVertex {
    fn new(position: cgmath::Vector3<f32>, texel: cgmath::Vector2<f32>) -> Self {
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
pub struct Geometry {
    pub vertices: Vec<BasicVertex>,
    pub indices: Vec<u32>,
}

impl Geometry {
    pub fn create_quad() -> Result<Geometry> {
        let vertices = [
            BasicVertex::new(vec3(-0.5, -0.5, 0.0), vec2(0.0, 0.0)),
            BasicVertex::new(vec3(0.5, -0.5, 0.0), vec2(1.0, 0.0)),
            BasicVertex::new(vec3(0.5, 0.5, 0.0), vec2(1.0, 1.0)),
            BasicVertex::new(vec3(-0.5, 0.5, 0.0), vec2(0.0, 1.0)),
        ];
        let indices = [0, 1, 2, 2, 3, 0];
        Ok(Geometry {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
        })
    }

    pub fn load_obj(path: &str) -> Result<Geometry> {
        // Model
        let mut reader = BufReader::new(File::open(path)?);

        let (models, _) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
            |_| Ok(Default::default()),
        )?;

        let mut vertices: Vec<BasicVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        // Vertices / Indices
        let mut unique_vertices = HashMap::new();

        for model in &models {
            for index in &model.mesh.indices {
                let pos_offset = (3 * index) as usize;
                let tex_coord_offset = (2 * index) as usize;

                let vertex = BasicVertex {
                    position: vec3(
                        model.mesh.positions[pos_offset],
                        model.mesh.positions[pos_offset + 1],
                        model.mesh.positions[pos_offset + 2],
                    ),
                    texel: vec2(
                        model.mesh.texcoords[tex_coord_offset],
                        1.0 - model.mesh.texcoords[tex_coord_offset + 1],
                    ),
                };

                if let Some(index) = unique_vertices.get(&vertex) {
                    indices.push(*index as u32);
                } else {
                    let index = vertices.len();
                    unique_vertices.insert(vertex, index);
                    vertices.push(vertex);
                    indices.push(index as u32);
                }
            }
        }
        Ok(Self { vertices, indices })
    }
}

impl Default for Geometry {
    #[inline]
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl fmt::Debug for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Texture.handle({:p}) - Texture.memory({:p})",
            self.vertices.len() as *const u8,
            self.indices.len() as *const u8
        )
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Mesh {
    device: Option<vulkanalia::Device>,
    pub geometry: Geometry,
    pub vertices: gpu::GPUBuffer,
    pub indices: gpu::GPUBuffer,
}

impl Mesh {
    pub fn create(device: &gpu::GPUDevice, geometry: &Geometry) -> Result<Mesh> {
        Ok(Self {
            device: Some(device.handle.api.logical_device.clone()),
            geometry: geometry.clone(),
            vertices: device
                .create_typed_buffer(GPUBufferUsageFlags::VERTEX, &geometry.vertices)?,
            indices: device.create_typed_buffer(GPUBufferUsageFlags::INDEX, &geometry.indices)?,
        })
    }

    pub fn destroy(&self) {
        // destroy vertices
        self.vertices.destroy();

        // destroy indices
        self.indices.destroy();
    }

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

impl Default for Mesh {
    #[inline]
    fn default() -> Self {
        Self {
            device: None,
            geometry: Geometry::default(),
            vertices: gpu::GPUBuffer::default(),
            indices: gpu::GPUBuffer::null(),
        }
    }
}
