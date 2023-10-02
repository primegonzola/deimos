// SPDX-License-Identifier: MIT

// #![allow(dead_code)]

use super::device::Device;
use super::{Buffer, Color, Pipeline, Viewport};
use anyhow::Result;
use std::cell::RefCell;
use std::sync::Arc;
use vulkano::pipeline::graphics;
use vulkano::render_pass::Framebuffer;

pub struct CommandBuffer {
    pub handle: Option<vulkano::command_buffer::PrimaryAutoCommandBuffer>,
    builder: RefCell<
        vulkano::command_buffer::AutoCommandBufferBuilder<
            vulkano::command_buffer::PrimaryAutoCommandBuffer,
        >,
    >,
}

impl CommandBuffer {
    pub fn begin(
        graphics: &Device,
        framebuffer: Arc<vulkano::render_pass::Framebuffer>,
        color: Option<Color>,
        depth: Option<f32>,
    ) -> Result<CommandBuffer> {
        // create the builder
        let builder = RefCell::new(
            vulkano::command_buffer::AutoCommandBufferBuilder::primary(
                &graphics.command_buffer_allocator,
                graphics.queue.queue_family_index(),
                vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap(),
        );

        // add the render pass
        builder
            .borrow_mut()
            // Before we can draw, we have to *enter a render pass*.
            .begin_render_pass(
                vulkano::command_buffer::RenderPassBeginInfo {
                    // A list of values to clear the attachments with. This list contains
                    // one item for each attachment in the render pass. In this case, there
                    // is only one attachment, and we clear it with a blue color.
                    //
                    // Only attachments that have `LoadOp::Clear` are provided with clear
                    // values, any others should use `ClearValue::None` as the clear value.
                    clear_values: vec![Some(color.unwrap().to_rgba().into())],

                    ..vulkano::command_buffer::RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                //
                // The contents of the first (and only) subpass. This can be either
                // `Inline` or `SecondaryCommandBuffers`. The latter is a bit more advanced
                // and is not covered here.
                //
                vulkano::command_buffer::SubpassContents::Inline,
            )
            .unwrap();
        // .set_viewport(0, [graphics.viewport.clone()])
        // .bind_pipeline_graphics(pipeline.handle.clone())
        // .bind_vertex_buffers(0, vertex_buffer.clone())
        // //
        // // We add a draw command.
        // //
        // .draw(vertex_buffer.len() as u32, 1, 0, 0)
        // .unwrap()
        //
        // We leave the render pass. Note that if we had multiple subpasses we could
        // have called `next_subpass` to jump to the next subpass.
        //
        // .end_render_pass()
        // .unwrap();

        // all done
        Ok(Self {
            handle: None,
            builder: builder,
        })
    }

    pub fn set_viewport(&self, viewport: Viewport) -> Result<()> {
        self.builder.borrow_mut().set_viewport(
            0,
            [vulkano::pipeline::graphics::viewport::Viewport {
                origin: viewport.origin.clone(),
                dimensions: viewport.dimensions.clone(),
                depth_range: viewport.depth_range.clone(),
            }],
        );
        Ok(())
    }

    pub fn bind_pipeline(&self, pipeline: &Pipeline) -> Result<()> {
        self.builder
            .borrow_mut()
            .bind_pipeline_graphics(pipeline.handle.clone());
        Ok(())
    }

    pub fn bind_vertex_buffer<T>(&self, vertex_buffer: &Buffer<T>) -> Result<()> {
        self.builder
            .borrow_mut()
            .bind_vertex_buffers(0, vertex_buffer.handle.clone());
        Ok(())
    }

    pub fn end(&mut self) -> Result<()> {
        //
        // We leave the render pass. Note that if we had multiple subpasses we could
        // have called `next_subpass` to jump to the next subpass.
        //
        self.builder.borrow_mut().end_render_pass().unwrap();
        Ok(())
    }
}
