// SPDX-License-Identifier: MIT

// #![allow(dead_code)]

// use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

// include the modules in the code graph
mod graphics;
mod rendering;

use graphics::Color;
use graphics::Pipeline;
use graphics::VertexPosition;
use graphics::{Buffer, CommandBuffer};

fn main() {
    // create an event loop
    let event_loop = EventLoop::new();

    // create graphics device
    let mut graphics =
        graphics::Device::create(&event_loop).expect("failed to create graphics device");

    // graphics.data.lock().unwrap().recreate_swapchain = true;

    // check https://stackoverflow.com/questions/68547268/cannot-borrow-data-in-an-arc-as-mutable

    // create vertices
    let vertices = [
        VertexPosition {
            position: [-0.5, -0.25],
        },
        VertexPosition {
            position: [0.0, 0.5],
        },
        VertexPosition {
            position: [0.25, -0.1],
        },
    ];

    // create vertex buffer
    let vertex_buffer = Buffer::from_iter(
        &graphics,
        vulkano::buffer::BufferUsage::VERTEX_BUFFER,
        vertices,
    )
    .unwrap();

    // let's create a standard pipeline
    let pipeline = Pipeline::create_standard_pipeline(&graphics).unwrap();

    // process the event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                graphics.recreate_swapchain = true;
            }
            Event::RedrawEventsCleared => {
                //
                // begin graphics frame
                //
                graphics
                    .begin_frame()
                    .expect("failed to begin graphics frame");

                // // create command buffer
                // let mut command_buffer =
                //     CommandBuffer::begin(&graphics, Some(Color::red()), Some(1.0)).unwrap();



                // // complete 
                // command_buffer.end(&graphics).unwrap();
                //
                // In order to draw, we have to build a *command buffer*. The command buffer object
                // holds the list of commands that are going to be executed.
                //
                // Building a command buffer is an expensive operation (usually a few hundred
                // microseconds), but it is known to be a hot path in the driver and is expected to
                // be optimized.
                //
                // Note that we have to pass a queue family when we create the command buffer. The
                // command buffer will only be executable on that given queue family.
                //
                let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
                    &graphics.command_buffer_allocator,
                    graphics.queue.queue_family_index(),
                    vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
                )
                .unwrap();

                builder
                    // Before we can draw, we have to *enter a render pass*.
                    .begin_render_pass(
                        vulkano::command_buffer::RenderPassBeginInfo {
                            // A list of values to clear the attachments with. This list contains
                            // one item for each attachment in the render pass. In this case, there
                            // is only one attachment, and we clear it with a blue color.
                            //
                            // Only attachments that have `LoadOp::Clear` are provided with clear
                            // values, any others should use `ClearValue::None` as the clear value.
                            clear_values: vec![Some(Color::blue().to_rgba().into())],

                            ..vulkano::command_buffer::RenderPassBeginInfo::framebuffer(
                                graphics.framebuffers[graphics.image_index as usize].clone(),
                            )
                        },
                        //
                        // The contents of the first (and only) subpass. This can be either
                        // `Inline` or `SecondaryCommandBuffers`. The latter is a bit more advanced
                        // and is not covered here.
                        //
                        vulkano::command_buffer::SubpassContents::Inline,
                    )
                    .unwrap()
                    // We are now inside the first subpass of the render pass.
                    //
                    // TODO: Document state setting and how it affects subsequent draw commands.
                    .set_viewport(0, [graphics.viewport.clone()])
                    .bind_pipeline_graphics(pipeline.handle.clone())
                    .bind_vertex_buffers(0, vertex_buffer.handle.clone())
                    //
                    // We add a draw command.
                    //
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    //
                    // We leave the render pass. Note that if we had multiple subpasses we could
                    // have called `next_subpass` to jump to the next subpass.
                    //
                    .end_render_pass()
                    .unwrap();

                // Finish building the command buffer by calling `build`.
                let command_buffer: vulkano::command_buffer::PrimaryAutoCommandBuffer =
                    builder.build().unwrap();

                
                //
                // end graphics frame
                //
                graphics
                    .end_frame(command_buffer)
                    .expect("failed to end graphics frame");
            }
            _ => (),
        }
    });
}
