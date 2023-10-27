// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

use std::str::FromStr;
use std::time::Instant;

use anyhow::Result;
use cgmath::{point3, vec3, Deg};
use winit::window::Window;

// use vulkanalia::prelude::v1_0::*;

use crate::gpu::{self};

pub struct Sample {
    graphics: gpu::GPUDevice,
    vertex_shader_module: gpu::GPUShaderModule,
    fragment_shader_module: gpu::GPUShaderModule,
    camera_uniform_buffer: gpu::GPUBuffer,
    albedo_texture: gpu::GPUTexture,
    albedo_view: gpu::GPUTextureView,
    albedo_sampler: gpu::GPUSampler,
    start: Instant,
    data: SampleData,
}

impl Sample {
    pub fn create(window: &Window) -> Result<Self> {
        // create data
        let mut data = SampleData::default();

        // create gpu device
        let graphics = gpu::GPUDevice::create(window)?;

        // create vertex shader module
        let vertex_shader_module =
            graphics.create_shader_module(gpu::GPUShaderModuleDescriptor {
                code: None,
                byte_code: Some(include_bytes!("../shaders/compiled/sample_vs.spv").to_vec()),
            })?;

        // create fragment shader module
        let fragment_shader_module =
            graphics.create_shader_module(gpu::GPUShaderModuleDescriptor {
                code: None,
                byte_code: Some(include_bytes!("../shaders/compiled/sample_fs.spv").to_vec()),
            })?;

        // create camera uniform
        let camera_uniform_buffer = graphics.create_typed_buffer::<gpu::GPUCameraUniform>(
            gpu::GPUBufferUsageFlags::UNIFORM,
            &vec![gpu::GPUCameraUniform::default()],
        )?;

        // load mesh
        data.mesh = gpu::Mesh::create(
            &graphics,
            &gpu::Geometry::load_obj("/Users/prime/depot/github/deimos/resources/viking_room.obj")?,
        )?;

        // load texture
        let albedo_texture =
            graphics.load_texture("/Users/prime/depot/github/deimos/resources/viking_room.png")?;

        // create view
        let albedo_view = albedo_texture.create_view(Some(gpu::GPUTextureViewDescriptor {
            format: None,
            dimension: None,
            aspect: None,
            base_mip_level: None,
            mip_level_count: Some(1),
            base_array_layer: None,
            array_layer_count: None,
        }))?;

        // create sampler
        let albedo_sampler = graphics.create_sampler(Some(gpu::GPUSamplerDescriptor {
            address_mode_u: Some(gpu::GPUAddressMode::ClampToEdge),
            address_mode_v: Some(gpu::GPUAddressMode::ClampToEdge),
            address_mode_w: Some(gpu::GPUAddressMode::ClampToEdge),
            mag_filter: Some(gpu::GPUFilterMode::Linear),
            min_filter: Some(gpu::GPUFilterMode::Linear),
            mipmap_filter: Some(gpu::GPUMipmapFilterMode::Nearest),
            lod_min_clamp: Some(0.0),
            lod_max_clamp: Some(100.0),
            compare: None,
            max_anisotropy: None,
        }))?;

        // all ok
        Ok(Self {
            graphics,
            vertex_shader_module,
            fragment_shader_module,
            camera_uniform_buffer,
            start: Instant::now(),
            albedo_texture,
            albedo_view,
            albedo_sampler,
            data,
        })
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        // check if any size if not we must be minimized
        if size.width != 0 && size.height != 0 {
            // mark graphics as resized
            self.graphics.resized(true).unwrap();
        }
    }

    pub fn get_current_camera(&self) -> Result<gpu::GPUCameraUniform> {
        // MVP
        let time = self.start.elapsed().as_secs_f32();
        let model = cgmath::Matrix4::from_axis_angle(vec3(0.0, 0.0, 1.0), Deg(90.0) * time);
        let view = cgmath::Matrix4::look_at_rh(
            point3::<f32>(2.0, 2.0, 2.0),
            point3::<f32>(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
        );
        #[rustfmt::skip]
        let correction = cgmath::Matrix4::new(
            1.0,  0.0,       0.0, 0.0,
            0.0, -1.0,       0.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 1.0,
        );
        let extent = self.graphics.get_current_extent()?;
        let projection = correction
            * cgmath::perspective(
                Deg(45.0),
                extent.width as f32 / extent.height.unwrap() as f32,
                0.1,
                10.0,
            );
        let ubo = gpu::GPUCameraUniform {
            model,
            view,
            projection,
        };

        // all done
        Ok(ubo)
    }

    fn begin_render_pass(
        &self,
        command_encoder: &gpu::GPUCommandEncoder,
    ) -> Result<gpu::GPURenderPassEncoder> {
        // begin render pass
        command_encoder.begin_render_pass(gpu::GPURenderPassDescriptor {
            color_attachments: Some(vec![gpu::GPURenderPassColorAttachment {
                view: self.graphics.get_back_albedo_view()?,
                load_op: gpu::GPULoadOp::Clear,
                store_op: gpu::GPUStoreOp::Store,
                clear_value: Some(gpu::GPUColor::from_vec(&[0.1, 0.2, 0.3, 1.0].to_vec())),
                resolve_target: None,
            }]),
            depth_stencil_attachment: Some(gpu::GPURenderPassDepthStencilAttachment {
                view: self.graphics.get_back_depth_view()?,
                depth_load_op: Some(gpu::GPULoadOp::Clear),
                depth_store_op: Some(gpu::GPUStoreOp::Store),
                depth_clear_value: Some(1.0),
                depth_read_only: None,
                stencil_load_op: None,
                stencil_store_op: None,
                stencil_clear_value: None,
                stencil_read_only: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
            max_draw_count: None,
        })
    }

    fn create_pipeline(
        &self,
    ) -> Result<(
        gpu::GPUPipelineLayout,
        gpu::GPURenderPipeline,
        gpu::GPUBindGroupLayout,
        gpu::GPUBindGroup,
    )> {
        // create bind group layout
        let bind_group_layout =
            self.graphics
                .create_bind_group_layout(gpu::GPUBindGroupLayoutDescriptor {
                    entries: vec![gpu::GPUBindGroupLayoutEntry {
                        binding: 0,
                        visibility: gpu::GPUShaderStageFlags::VERTEX
                            | gpu::GPUShaderStageFlags::FRAGMENT,
                        buffer: Some(gpu::GPUBufferBindingLayout {
                            r#type: Some(gpu::GPUBufferBindingType::Uniform),
                            has_dynamic_offset: Some(false),
                            min_binding_size: None,
                        }),
                        sampler: None,
                        texture: None,
                        storage_texture: None,
                        external_texture: None,
                    }],
                })?;

        // create bind group
        let camera_bind_group = self
            .graphics
            .create_bind_group(gpu::GPUBindGroupDescriptor {
                layout: bind_group_layout,
                entries: vec![gpu::GPUBindGroupEntry {
                    binding: 0,
                    resource: gpu::GPUBindingResource {
                        buffer_binding: Some(gpu::GPUBufferBinding {
                            buffer: self.camera_uniform_buffer.clone(),
                            offset: Some(0),
                            size: None,
                        }),
                        sampler: None,
                        texture_view: None,
                        external_texture: None,
                    },
                }],
            })?;

        // create pipeline layout
        let pipeline_layout: gpu::GPUPipelineLayout =
            self.graphics
                .create_pipeline_layout(gpu::GPUPipelineLayoutDescriptor {
                    bind_group_layouts: vec![bind_group_layout],
                })?;

        // create pipeline
        let pipeline = self
            .graphics
            .create_render_pipeline(gpu::GPURenderPipelineDescriptor {
                layout: Some(pipeline_layout),
                vertex: gpu::GPUVertexState {
                    module: self.vertex_shader_module,
                    entry_point: String::from_str("main")?,
                    constants: None,
                    buffers: Some(vec![gpu::GPUVertexBufferLayout {
                        array_stride: 3 * std::mem::size_of::<f32>() as u64,
                        step_mode: Some(gpu::GPUVertexStepMode::Vertex),
                        attributes: vec![
                            gpu::GPUVertexAttribute {
                                format: gpu::GPUVertexFormat::Float32x3,
                                offset: 0,
                                shader_location: 0,
                            },
                            gpu::GPUVertexAttribute {
                                format: gpu::GPUVertexFormat::Float32x3,
                                offset: 3 * std::mem::size_of::<f32>() as u64,
                                shader_location: 1,
                            },
                        ],
                    }]),
                },
                primitive: Some(gpu::GPUPrimitiveState {
                    topology: Some(gpu::GPUPrimitiveTopology::TriangleList),
                    strip_index_format: None,
                    front_face: Some(gpu::GPUFrontFace::CCW),
                    cull_mode: Some(gpu::GPUCullMode::Back),
                    unclipped_depth: None,
                }),
                depth_stencil: Some(gpu::GPUDepthStencilState {
                    format: gpu::GPUTextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: gpu::GPUCompareFunction::Less,
                    stencil_front: None,
                    stencil_back: None,
                    stencil_read_mask: None,
                    stencil_write_mask: None,
                    depth_bias: None,
                    depth_bias_clamp: None,
                    depth_bias_slope_scale: None,
                }),
                multisample: None,
                fragment: Some(gpu::GPUFragmentState {
                    module: self.fragment_shader_module,
                    entry_point: String::from_str("main")?,
                    constants: None,
                    targets: Some(vec![gpu::GPUColorTargetState {
                        format: self.graphics.get_presentation_format()?,
                        blend: Some(gpu::GPUBlendState {
                            color: gpu::GPUBlendComponent {
                                src_factor: Some(gpu::GPUBlendFactor::One),
                                dst_factor: Some(gpu::GPUBlendFactor::Zero),
                                operation: Some(gpu::GPUBlendOperation::Add),
                            },
                            alpha: gpu::GPUBlendComponent {
                                src_factor: Some(gpu::GPUBlendFactor::One),
                                dst_factor: Some(gpu::GPUBlendFactor::Zero),
                                operation: Some(gpu::GPUBlendOperation::Add),
                            },
                        }),
                        write_mask: None,
                    }]),
                }),
            })?;
        Ok((
            pipeline_layout,
            pipeline,
            bind_group_layout,
            camera_bind_group,
        ))
    }

    fn render_scene(
        &self,
        render_pass_encoder: &gpu::GPURenderPassEncoder,
        pipeline: &gpu::GPURenderPipeline,
        camera_bind_group: gpu::GPUBindGroup,
    ) -> Result<()> {
        // bind camera group
        render_pass_encoder.set_bind_group(0, &camera_bind_group, None)?;

        // set pipeline
        render_pass_encoder.set_pipeline(&pipeline)?;

        // set vertex buffer
        render_pass_encoder.set_vertex_buffer(0, &self.data.mesh.vertices, None, None)?;

        // set index buffer
        render_pass_encoder.set_index_buffer(
            &self.data.mesh.indices,
            gpu::GPUIndexFormat::Uint32,
            None,
            None,
        )?;

        // draw indexed
        render_pass_encoder.draw_indexed(
            self.data.mesh.geometry.indices.len() as u32,
            None,
            None,
            None,
            None,
        )?;

        Ok(())
    }

    fn render_contents(&self) -> Result<()> {
        // create queue
        let queue = self.graphics.create_queue()?;

        // create command encoder
        let command_encoder = self.graphics.create_command_encoder(None).unwrap();

        // begin render pass
        let render_pass_encoder = self.begin_render_pass(&command_encoder)?;

        // create pipeline
        let (pipeline_layout, pipeline, bind_group_layout, camera_bind_group) =
            self.create_pipeline()?;

        // render the scene
        self.render_scene(&render_pass_encoder, &pipeline, camera_bind_group)?;

        // end the render pass
        render_pass_encoder.end()?;

        // finish command encoder
        let command_buffer = command_encoder.finish(None)?;

        // submit command buffer
        queue.submit(&[command_buffer])?;

        // destroy command buffer
        command_buffer.destroy();

        // destroy pipeline
        pipeline.destroy();

        // destroy pipeline layout
        pipeline_layout.destroy();

        // destroy camera bind group
        camera_bind_group.destroy();

        // destroy bind group layout
        bind_group_layout.destroy();

        // destroy the render pass
        render_pass_encoder.destroy();

        // destroy queue
        queue.destroy();

        Ok(())
    }

    pub fn update(&mut self, window: &Window) -> Result<()> {
        // check if valids
        if !self.graphics.is_valid()? {
            // notifys
            print!("Invalid graphics, resetting ...\n");

            // reset the graphics
            self.graphics.reset(window)?;
        } else {
            // let's start the frame
            self.graphics.begin_frame(window)?;

            // update camera buffer
            self.graphics.write_buffer(
                &self.camera_uniform_buffer,
                0,
                &vec![self.get_current_camera()?],
            )?;

            // render the contents
            self.render_contents()?;

            // self.graphics.present(window, Sample::test)?;

            // end the current frame
            self.graphics.end_frame(window)?;
        }

        // all fine
        Ok(())
    }

    pub fn test() {}

    pub fn destroy(&mut self) {
        // destroy data
        self.data.destroy();

        // destroy vertex shader module
        self.vertex_shader_module.destroy();

        // destroy fragment shader module
        self.fragment_shader_module.destroy();

        // destroy camera uniform buffer
        self.camera_uniform_buffer.destroy();

        // destroy albedo texture
        self.albedo_texture.destroy();

        // destroy albedo view
        self.albedo_view.destroy();

        // destroy albedo sampler
        self.albedo_sampler.destroy();

        // destroy graphics
        self.graphics.destroy();
    }
}

#[derive(Clone, Default)]
struct SampleData {
    mesh: gpu::Mesh,
}

impl SampleData {
    fn destroy(&mut self) {
        // destroy mesh
        self.mesh.destroy();
    }
}
