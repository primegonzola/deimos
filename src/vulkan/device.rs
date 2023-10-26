// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use std::time::Instant;

use anyhow::{anyhow, Result};
use vulkanalia::prelude::v1_0::*;
use winit::window::Window;

use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;

use super::{
    CameraUniform, Mesh, Swapchain, Texture, TextureView, VulkanApi, VulkanDeviceSync,
    MAX_DESCRIPTOR_SETS, MAX_FRAMES_IN_FLIGHT, VALIDATION_ENABLED,
};

#[derive(Clone)]
pub struct VulkanDevice {
    pub api: VulkanApi,
    pub data: VulkanDeviceData,
    pub frame_index: usize,
    pub swap_index: usize,
    pub resized: bool,
    pub start: Instant,
    sync: VulkanDeviceSync,
    messenger: Option<vk::DebugUtilsMessengerEXT>,
}

impl VulkanDevice {
    pub fn is_valid(&self) -> Result<bool> {
        Ok(self.resized == false)
    }

    pub fn create(window: &Window) -> Result<Self> {
        unsafe {
            // start with no data
            let mut data = VulkanDeviceData::default();

            // get entry
            let entry = VulkanApi::get_entry()?;

            // get instance and messenger
            let (instance, messenger) = VulkanApi::create_instance(&entry, window, "D E I M O S")?;

            // create surface
            data.surface = VulkanApi::create_surface(&instance, &window, &window)?;

            // get physical device and samples
            let (physical_device, samples) =
                VulkanApi::select_physical_device(&instance, data.surface)?;

            // save samples
            data.samples = samples;

            // create logical device etc
            let (logical_device, graphics_queue, present_queue) = VulkanApi::create_logical_device(
                entry.clone(),
                instance.clone(),
                physical_device,
                data.surface,
            )?;

            // overwrite queues
            data.graphics_queue = graphics_queue;
            data.present_queue = present_queue;

            // create api
            let api = VulkanApi {
                entry,
                instance,
                logical_device: logical_device.clone(),
                physical_device,
            };

            // create swapchain
            data.swapchain = Swapchain::create(&api, data.surface, window)?;

            // create color objects
            (data.blit_albedo_texture, data.blit_albedo_view) = create_color_objects(
                &api,
                data.swapchain.format,
                data.swapchain.extent,
                data.samples,
            )?;

            // create depth objects
            (data.blit_depth_texture, data.blit_depth_view) =
                create_depth_objects(&api, data.swapchain.extent, data.samples)?;

            // create uniform buffers
            data.uniform_buffers =
                api.create_uniform_buffers::<CameraUniform>(data.swapchain.views.len())?;

            // create render pass
            data.render_pass = api.create_render_pass(data.swapchain.format, data.samples)?;

            // create descriptor set layout
            data.descriptor_set_layout = api.create_descriptor_set_layout()?;

            // create pipeline layout
            data.pipeline_layout = api.create_pipeline_layout(data.descriptor_set_layout)?;

            // create pipeline
            data.pipeline = api.create_pipeline(
                data.render_pass,
                data.pipeline_layout,
                data.swapchain.extent,
                data.samples,
                Mesh::binding_description(),
                Mesh::attribute_descriptions().to_vec(),
            )?;
            // create framebuffers
            data.framebuffers = api.create_framebuffers(
                data.render_pass,
                data.swapchain.extent,
                data.blit_albedo_view.handle,
                data.blit_depth_view.handle,
                &data.swapchain.views.iter().map(|v| v.handle).collect(),
            )?;

            // create the command pool
            data.command_pool = api.create_command_pool(data.surface)?;

            // load albedo texture
            let (image, memory, mip_levels) = api.load_image(
                data.graphics_queue,
                data.command_pool,
                "/Users/prime/depot/github/deimos/resources/viking_room.png",
            )?;

            data.back_albedo_texture = Texture {
                handle: image,
                memory,
                mip_levels,
            };

            // create the albedo view
            data.back_albedo_view = data.back_albedo_texture.create_view(
                &api,
                vk::Format::R8G8B8A8_SRGB,
                vk::ImageAspectFlags::COLOR,
                data.back_albedo_texture.mip_levels,
            )?;

            create_texture_sampler(
                &logical_device,
                data.back_albedo_texture.mip_levels,
                &mut data,
            )?;

            // create descriptor pool
            data.descriptor_pool = api.create_descriptor_pool(MAX_DESCRIPTOR_SETS)?;

            // create descriptor sets
            data.descriptor_sets = api.create_descriptor_sets(
                data.descriptor_pool,
                data.descriptor_set_layout,
                data.swapchain.views.len(),
            )?;

            // update descriptor sets
            api.update_descriptor_sets::<CameraUniform>(
                &data.descriptor_sets,
                &data.uniform_buffers,
                data.back_albedo_view.handle,
                data.back_albedo_sampler,
                data.swapchain.views.len(),
            )?;

            // create command buffers
            data.command_buffers =
                api.create_command_buffers(data.command_pool, data.swapchain.views.len())?;

            // create command buffer queues
            data.command_buffer_queues = Vec::new();
            data.swapchain.views.iter().for_each(|_| {
                data.command_buffer_queues.push(Vec::new());
            });

            // create sync object
            let sync = VulkanDeviceSync::create(
                &logical_device,
                MAX_FRAMES_IN_FLIGHT,
                data.swapchain.views.len(),
            )?;

            Ok(Self {
                api,
                data,
                frame_index: 0,
                swap_index: 0,
                resized: false,
                messenger,
                sync,
                start: Instant::now(),
            })
        }
    }

    pub fn begin_frame(&mut self, window: &Window) -> Result<()> {
        unsafe {
            self.sync.in_flight_fence = self.sync.fences_in_flight[self.frame_index];

            self.api.logical_device.wait_for_fences(
                &[self.sync.in_flight_fence],
                true,
                u64::MAX,
            )?;

            let result = self.api.logical_device.acquire_next_image_khr(
                self.data.swapchain.handle,
                u64::MAX,
                self.sync.image_available_semaphores[self.frame_index],
                vk::Fence::null(),
            );

            // get next
            self.swap_index = match result {
                Ok((image_index, _)) => image_index as usize,
                Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
                Err(e) => return Err(anyhow!(e)),
            };

            let image_in_flight = self.sync.textures_in_flight[self.swap_index];
            if !image_in_flight.is_null() {
                self.api
                    .logical_device
                    .wait_for_fences(&[image_in_flight], true, u64::MAX)?;
            }
            // move next in flight fence to current
            self.sync.textures_in_flight[self.swap_index] = self.sync.in_flight_fence;

            // all done
            Ok(())
        }
    }

    pub fn clear_frame(&mut self, color: Option<[f32; 4]>, depth: Option<f32>) -> Result<()> {
        // begin render pass
        self.begin_render_pass(
            self.data.render_pass,
            self.data.framebuffers[self.swap_index],
            self.data.swapchain.extent,
            self.data.command_buffers[self.swap_index],
            color,
            depth,
        )?;

        // end render pass
        self.end_render_pass(self.data.command_buffers[self.swap_index])?;

        // end command buffer
        self.api
            .end_command_buffer(self.data.command_buffers[self.swap_index])?;

        // add command to queue
        self.data
            .command_buffer_queues
            .get_mut(self.swap_index)
            .unwrap()
            .push(self.data.command_buffers[self.swap_index]);

        // all done
        Ok(())
    }

    pub fn end_frame(&mut self, window: &Window) -> Result<()> {
        unsafe {
            // create command buffers to submit for rendering
            // let command_buffers = &[self.data.command_buffers[self.swap_index]];
            // add to command buffer queue

            // // print len
            // if self.data.command_buffer_queues[self.swap_index].len() > 0 {
            //     println!(
            //         "VKDevice::end_frame_internal\t\t\tbuffer: 0x{:x}",
            //         self.data.command_buffer_queues[self.swap_index][0].as_raw()
            //     );
            // }

            // check anything in queue
            if self.data.command_buffer_queues[self.swap_index].len() == 0 {
                // clear frame
                self.clear_frame(Some([0.075, 0.075, 0.075, 1.0]), Some(1.0))?;
            }

            // assume no change
            let mut changed = false;

            // check if any
            if self.data.command_buffer_queues[self.swap_index].len() > 0 {
                // println!(
                //     "VKDevice::end_frame\t\t\t\tbuffer: 0x{:x}",
                //     self.data.command_buffer_queues[self.swap_index][0].as_raw()
                // );

                // consolidate command buffers
                let command_buffers = &self.data.command_buffer_queues[self.swap_index];
                // all the rest is waiting for buffers and output generated
                let wait_semaphores = &[self.sync.image_available_semaphores[self.frame_index]];
                let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
                let signal_semaphores = &[self.sync.render_finished_semaphores[self.frame_index]];
                let submit_info = vk::SubmitInfo::builder()
                    .wait_semaphores(wait_semaphores)
                    .wait_dst_stage_mask(wait_stages)
                    .command_buffers(command_buffers)
                    .signal_semaphores(signal_semaphores);

                // reset in flight fence
                self.api
                    .logical_device
                    .reset_fences(&[self.sync.in_flight_fence])?;

                // all ready to submit
                self.api.logical_device.queue_submit(
                    self.data.graphics_queue,
                    &[submit_info],
                    self.sync.in_flight_fence,
                )?;

                let swapchains = &[self.data.swapchain.handle];
                let image_indices = &[self.swap_index as u32];
                let present_info = vk::PresentInfoKHR::builder()
                    .wait_semaphores(signal_semaphores)
                    .swapchains(swapchains)
                    .image_indices(image_indices);

                let result = self
                    .api
                    .logical_device
                    .queue_present_khr(self.data.present_queue, &present_info);

                // update changed
                changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
                    || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

                // clear command buffer queue
                self.data.command_buffer_queues[self.swap_index].clear();

                // update the frame for next update
                self.frame_index = (self.frame_index + 1) % MAX_FRAMES_IN_FLIGHT;
            }
            // always check swapchain
            // see if either a resize, a suboptimal or out of date
            // trigger a recreation of the swapchain if so
            if self.resized || changed {
                self.resized = false;
                self.recreate_swapchain(window)?;
            }

            // all done
            Ok(())
        }
    }

    pub fn reset(&mut self, window: &Window) -> Result<()> {
        if self.resized {
            self.resized = false;
            self.recreate_swapchain(window)?;
        }
        // all done
        Ok(())
    }

    pub fn destroy(&mut self) {
        unsafe {
            // wait until idle
            self.wait_until_idle();

            // destroy swapchain
            self.destroy_swapchain();

            // destroy sync objects
            self.sync.destroy(&self.api.logical_device);

            // destroy sampler
            self.api
                .logical_device
                .destroy_sampler(self.data.back_albedo_sampler, None);

            // destroy albedo view & texture
            self.data.back_albedo_view.destroy(self);
            self.data.back_albedo_texture.destroy(self);

            // destroy command pool
            self.api
                .logical_device
                .destroy_command_pool(self.data.command_pool, None);

            // destroy descriptor layout
            self.api
                .logical_device
                .destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);

            // destroy logical device
            self.api.logical_device.destroy_device(None);

            // destroy surface
            self.api
                .instance
                .destroy_surface_khr(self.data.surface, None);

            // if validation was enabled we want to clean up the messenger
            if VALIDATION_ENABLED && self.messenger.is_some() {
                self.api
                    .instance
                    .destroy_debug_utils_messenger_ext(self.messenger.unwrap(), None);
            }

            // last is detroying the instance
            self.api.instance.destroy_instance(None);
        }
    }

    pub fn get_current_extent(&self) -> vk::Extent2D {
        self.data.swapchain.extent
    }

    pub fn get_current_format(&self) -> vk::Format {
        self.data.swapchain.format
    }

    pub fn wait_until_idle(&self) {
        unsafe { self.api.logical_device.device_wait_idle().unwrap() }
    }

    fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        // wait until idle
        self.wait_until_idle();

        // destroy the swapchain
        self.destroy_swapchain();

        // create new swapchain
        self.data.swapchain = Swapchain::create(&self.api, self.data.surface, window)?;

        // create color objects
        (self.data.blit_albedo_texture, self.data.blit_albedo_view) = create_color_objects(
            &self.api,
            self.data.swapchain.format,
            self.data.swapchain.extent,
            self.data.samples,
        )?;

        // create depth objects
        (self.data.blit_depth_texture, self.data.blit_depth_view) =
            create_depth_objects(&self.api, self.data.swapchain.extent, self.data.samples)?;

        // create uniform buffers
        self.data.uniform_buffers = self
            .api
            .create_uniform_buffers::<CameraUniform>(self.data.swapchain.views.len())?;

        // create render pass
        self.data.render_pass = self
            .api
            .create_render_pass(self.data.swapchain.format, self.data.samples)?;

        // create the pipeline layout
        self.data.pipeline_layout = self
            .api
            .create_pipeline_layout(self.data.descriptor_set_layout)?;

        // create pipeline
        self.data.pipeline = self.api.create_pipeline(
            self.data.render_pass,
            self.data.pipeline_layout,
            self.data.swapchain.extent,
            self.data.samples,
            Mesh::binding_description(),
            Mesh::attribute_descriptions().to_vec(),
        )?;

        // create framebuffers
        self.data.framebuffers = self.api.create_framebuffers(
            self.data.render_pass,
            self.data.swapchain.extent,
            self.data.blit_albedo_view.handle,
            self.data.blit_depth_view.handle,
            &self.data.swapchain.views.iter().map(|v| v.handle).collect(),
        )?;

        // create descriptor pool
        self.data.descriptor_pool = self.api.create_descriptor_pool(MAX_DESCRIPTOR_SETS)?;

        // create descriptor sets
        self.data.descriptor_sets = self.api.create_descriptor_sets(
            self.data.descriptor_pool,
            self.data.descriptor_set_layout,
            self.data.swapchain.views.len(),
        )?;

        // update descriptor sets
        self.api.update_descriptor_sets::<CameraUniform>(
            &self.data.descriptor_sets,
            &self.data.uniform_buffers,
            self.data.back_albedo_view.handle,
            self.data.back_albedo_sampler,
            self.data.swapchain.views.len(),
        )?;

        // create command buffers
        self.data.command_buffers = self
            .api
            .create_command_buffers(self.data.command_pool, self.data.swapchain.views.len())?;

        // create command buffer queues
        self.data.command_buffer_queues = Vec::new();
        self.data.swapchain.views.iter().for_each(|_| {
            self.data.command_buffer_queues.push(Vec::new());
        });

        //
        // update the textures in flight length
        // so we are ready for next update
        //
        self.sync
            .textures_in_flight
            .resize(self.data.swapchain.textures.len(), vk::Fence::null());

        Ok(())
    }

    fn destroy_swapchain(&mut self) {
        unsafe {
            // free the command buffers
            self.api
                .logical_device
                .free_command_buffers(self.data.command_pool, &self.data.command_buffers);

            self.api
                .logical_device
                .destroy_descriptor_pool(self.data.descriptor_pool, None);

            // destroy uniform buffers
            self.data.uniform_buffers.iter().for_each(|b| {
                self.api.logical_device.destroy_buffer(b.0, None);
                self.api.logical_device.free_memory(b.1, None);
            });

            // destroy color view & texture
            self.data.blit_albedo_view.destroy(self);
            self.data.blit_albedo_texture.destroy(self);

            // destroy depth view & texture
            self.data.blit_depth_view.destroy(self);
            self.data.blit_depth_texture.destroy(self);

            // destroy framebuffers
            self.data
                .framebuffers
                .iter()
                .for_each(|f| self.api.logical_device.destroy_framebuffer(*f, None));

            self.api
                .logical_device
                .destroy_pipeline(self.data.pipeline, None);

            self.api
                .logical_device
                .destroy_pipeline_layout(self.data.pipeline_layout, None);

            self.api
                .logical_device
                .destroy_render_pass(self.data.render_pass, None);

            self.data.swapchain.destroy(self);
        }
    }

    pub fn begin_render_pass(
        &self,
        render_pass: vk::RenderPass,
        framebuffer: vk::Framebuffer,
        extent: vk::Extent2D,
        command_buffer: vk::CommandBuffer,
        color: Option<[f32; 4]>,
        depth: Option<f32>,
    ) -> Result<()> {
        unsafe {
            // Commands
            let info = vk::CommandBufferBeginInfo::builder();

            // reset command buffer
            self.api.reset_command_buffer(command_buffer)?;

            // begin command buffer
            self.api
                .logical_device
                .begin_command_buffer(command_buffer, &info)?;

            // get render area
            let render_area = vk::Rect2D::builder()
                .offset(vk::Offset2D::default())
                .extent(extent);

            // create render pass info
            let mut info = vk::RenderPassBeginInfo::builder()
                .render_pass(render_pass)
                .framebuffer(framebuffer)
                .render_area(render_area);

            // check if clearing is required
            let color_clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: color.unwrap_or([0.0, 1.0, 1.0, 1.0]),
                },
            };
            let depth_clear_value = vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: depth.unwrap_or(1.0),
                    stencil: 0,
                },
            };

            let clear_values = &[color_clear_value, depth_clear_value];

            // clear value
            info = info.clear_values(clear_values);

            // begin render pass
            self.api.logical_device.cmd_begin_render_pass(
                command_buffer,
                &info,
                vk::SubpassContents::INLINE,
            );

            // all done
            Ok(())
        }
    }

    pub fn end_render_pass(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        unsafe {
            // end render pass
            self.api.logical_device.cmd_end_render_pass(command_buffer);

            Ok(())
        }
    }

    pub fn submit_command_buffers(
        &mut self,
        command_buffers: &Vec<vk::CommandBuffer>,
    ) -> Result<()> {
        command_buffers.iter().for_each(|c| {
            self.data.command_buffer_queues[self.swap_index].push(*c);
        });
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct VulkanDeviceData {
    // texture & view & sampler
    pub back_albedo_texture: Texture,
    pub back_albedo_view: TextureView,
    pub back_albedo_sampler: vk::Sampler,

    // color & depth textures
    pub blit_albedo_texture: Texture,
    pub blit_albedo_view: TextureView,
    pub blit_depth_texture: Texture,
    pub blit_depth_view: TextureView,

    // surface
    pub surface: vk::SurfaceKHR,
    pub samples: vk::SampleCountFlags,

    // queues
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,

    // swapchain
    pub swapchain: Swapchain,

    // pipeline layout
    pub render_pass: vk::RenderPass,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,

    // framebuffers
    pub framebuffers: Vec<vk::Framebuffer>,

    // command Pool
    pub command_pool: vk::CommandPool,

    // uniform buffers
    pub uniform_buffers: Vec<(vk::Buffer, vk::DeviceMemory)>,

    // descriptors
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_sets: Vec<vk::DescriptorSet>,

    // command buffers
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub command_buffer_queues: Vec<Vec<vk::CommandBuffer>>,
}

fn create_color_objects(
    api: &VulkanApi,
    format: vk::Format,
    extent: vk::Extent2D,
    samples: vk::SampleCountFlags,
) -> Result<(Texture, TextureView)> {
    // create texture
    let texture = Texture::create(
        api,
        extent.width,
        extent.height,
        1,
        samples,
        format,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSIENT_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // create view
    let view = texture.create_view(api, format, vk::ImageAspectFlags::COLOR, 1)?;

    // all done
    Ok((texture, view))
}

fn create_depth_objects(
    api: &VulkanApi,
    extent: vk::Extent2D,
    samples: vk::SampleCountFlags,
) -> Result<(Texture, TextureView)> {
    // get supported depth format
    let format = api.get_depth_format()?;
    // create texture
    let texture = Texture::create(
        api,
        extent.width,
        extent.height,
        1,
        samples,
        format,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // create view
    let view = texture.create_view(api, format, vk::ImageAspectFlags::DEPTH, 1)?;

    // all done
    Ok((texture, view))
}

unsafe fn create_texture_sampler(
    device: &vulkanalia::Device,
    mip_levels: u32,
    data: &mut VulkanDeviceData,
) -> Result<()> {
    let info = vk::SamplerCreateInfo::builder()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::REPEAT)
        .address_mode_v(vk::SamplerAddressMode::REPEAT)
        .address_mode_w(vk::SamplerAddressMode::REPEAT)
        .anisotropy_enable(true)
        .max_anisotropy(16.0)
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false)
        .compare_enable(false)
        .compare_op(vk::CompareOp::ALWAYS)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .min_lod(0.0)
        .max_lod(mip_levels as f32)
        .mip_lod_bias(0.0);

    data.back_albedo_sampler = device.create_sampler(&info, None)?;

    Ok(())
}
