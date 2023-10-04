// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use super::{Buffer, Pipeline, Viewport};
use std::sync::Arc;

pub struct CommandBuffer<V> {
    pub commands: Vec<CommandData<V>>,
}

impl<V> CommandBuffer<V> {
    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.commands.push(CommandData {
            viewport: Some(viewport.clone()),
            pipeline: None,
            vertex_buffer: None,
            draw: None,
        });
    }
    pub fn bind_pipeline(&mut self, pipeline: Arc<Pipeline>) {
        self.commands.push(CommandData {
            viewport: None,
            pipeline: Some(pipeline.clone()),
            vertex_buffer: None,
            draw: None,
        });
    }
    pub fn bind_vertex_buffer(&mut self, vertex_buffer: Arc<Buffer<V>>) {
        self.commands.push(CommandData {
            viewport: None,
            pipeline: None,
            vertex_buffer: Some(vertex_buffer.clone()),
            draw: None,
        });
    }

    pub fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self.commands.push(CommandData {
            viewport: None,
            pipeline: None,
            vertex_buffer: None,
            draw: Some([vertex_count, instance_count, first_vertex, first_instance]),
        });
    }
}

pub struct CommandData<V> {
    pub viewport: Option<Viewport>,
    pub pipeline: Option<Arc<Pipeline>>,
    pub vertex_buffer: Option<Arc<Buffer<V>>>,
    pub draw: Option<[u32; 4]>,
}
