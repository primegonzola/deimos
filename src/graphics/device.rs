// SPDX-License-Identifier: MIT

#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use std::collections::{HashMap, HashSet};
use std::ffi::CStr;
use std::fs::File;
use std::io::BufReader;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::{copy_nonoverlapping as memcpy, slice_from_raw_parts};
use std::time::Instant;

use anyhow::{anyhow, Result};
use cgmath::{point3, vec2, vec3, Deg};
use log::*;
use thiserror::Error;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::Version;
use winit::window::Window;

use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;

use super::Buffer;
use super::CommandBuffer;
use super::CommandPool;
use super::DescriptorPool;
use super::DescriptorSet;
use super::FrameBuffer;
use super::Queue;
use super::RenderPass;
use super::Sampler;
use super::Shader;
use super::Texture;
use super::TextureView;
use super::UniformBufferObject;
use super::Vertex;

// Whether the validation layers should be enabled.
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
// The name of the validation layers.
const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

// The required device extensions.
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

// The maximum number of frames that can be processed concurrently.
const MAX_FRAMES_IN_FLIGHT: usize = 2;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;
type Mat4 = cgmath::Matrix4<f32>;

/// Our the app.
#[derive(Clone, Debug)]
pub struct GraphicsDevice {
    entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    physical: vk::PhysicalDevice,
    samples: vk::SampleCountFlags,
    device: Device,
    data: GraphicsDeviceData,
    pub frame: usize,
    pub resized: bool,
    pub start: Instant,
}

impl GraphicsDevice {
    /// Creates our the app.
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = GraphicsDeviceData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        let surface = vk_window::create_surface(&instance, &window, &window)?;
        let physical = pick_physical_device(&instance, &surface, &mut data)?;
        let samples = get_max_msaa_samples(&instance, &physical);
        let device = create_logical_device(&entry, &instance, &surface, &physical, &mut data)?;

        create_swapchain(window, &instance, &surface, &physical, &device, &mut data)?;
        create_swapchain_views(&device, &mut data)?;

        create_render_pass(&instance, &physical, &samples, &device, &mut data)?;
        create_descriptor_set_layout(&device, &mut data)?;

        create_pipeline(&device, &samples, &mut data)?;
        create_command_pools(&instance, &surface, &physical, &device, &mut data)?;
        create_color_objects(&instance, &physical, &samples, &device, &mut data)?;
        create_depth_objects(&instance, &physical, &samples, &device, &mut data)?;
        create_framebuffers(&device, &mut data)?;
        create_texture_image(&instance, &physical, &device, &mut data)?;
        create_texture_image_view(&device, &mut data)?;
        create_texture_sampler(&device, &mut data)?;
        load_model(&mut data)?;
        create_vertex_buffer(&instance, &physical, &device, &mut data)?;
        create_index_buffer(&instance, &physical, &device, &mut data)?;
        create_uniform_buffers(&instance, &physical, &device, &mut data)?;
        create_descriptor_pool(&device, &mut data)?;
        create_descriptor_sets(&device, &mut data)?;
        create_command_buffers(&device, &mut data)?;
        create_sync_objects(&device, &mut data)?;
        Ok(Self {
            entry,
            instance,
            surface,
            samples,
            physical,
            device,
            data,
            frame: 0,
            resized: false,
            start: Instant::now(),
        })
    }

    /// update the app.
    pub unsafe fn update(&mut self, window: &Window, count: usize) -> Result<()> {
        // create an in flight fence to wait for
        let in_flight_fence = self.data.in_flight_fences[self.frame];

        // wait for the fence
        self.device
            .wait_for_fences(&[in_flight_fence], true, u64::max_value())?;

        // get next image
        let result = self.device.acquire_next_image_khr(
            self.data.swapchain,
            u64::max_value(),
            self.data.textures_available_semaphores[self.frame],
            vk::Fence::null(),
        );

        // get the image or rebuild if not found
        let image_index = match result {
            Ok((image_index, _)) => image_index as usize,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
            Err(e) => return Err(anyhow!(e)),
        };

        // get the current image to use
        let texture_in_flight = self.data.textures_in_flight[image_index];

        // check if valid
        if !texture_in_flight.is_null() {
            // wait for it until it is valid
            self.device
                .wait_for_fences(&[texture_in_flight], true, u64::max_value())?;
        }

        // set next image to use
        self.data.textures_in_flight[image_index] = in_flight_fence;

        // update command buffer
        self.update_command_buffer(image_index, count)?;

        // update uniform buffer
        self.update_uniform_buffer(image_index)?;

        let wait_semaphores = &[self.data.textures_available_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.primary_command_buffers[image_index].buffer];
        let signal_semaphores = &[self.data.render_finished_semaphores[self.frame]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        // reset all fences
        self.device.reset_fences(&[in_flight_fence])?;

        // submit buffers to que
        self.device.queue_submit(
            self.data.graphics_queue.queue,
            &[submit_info],
            in_flight_fence,
        )?;

        // get the swapchain
        let swapchains = &[self.data.swapchain];

        // image index to present
        let image_indices = &[image_index as u32];

        // get the present infoe
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        // get the current presentation info
        let result = self
            .device
            .queue_present_khr(self.data.present_queue.queue, &present_info);

        // check if changed or resized
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
            || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

        // check if resizing occurred and if resize the swapchain
        if self.resized || changed {
            // reset resized status
            self.resized = false;

            // recreate the swapchain
            self.recreate_swapchain(window)?;
        } else if let Err(e) = result {
            // handle error
            return Err(anyhow!(e));
        }

        // update frame counter
        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        // all went fine
        Ok(())
    }

    /// Updates a command buffer for our the app.
    // #[rustfmt::skip]
    unsafe fn update_command_buffer(&mut self, index: usize, count: usize) -> Result<()> {
        // reset command pool
        self.device.reset_command_pool(
            self.data.command_pools[index].pool,
            vk::CommandPoolResetFlags::empty(),
        )?;

        // get the command buffer associated
        let command_buffer = self.data.primary_command_buffers[index];

        // prepare command info
        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        // begin the command
        self.device
            .begin_command_buffer(command_buffer.buffer, &info)?;

        // define render area
        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(self.data.swapchain_extent);

        // define clear value used for color
        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };

        // define clear value used for depth
        let depth_clear_value = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            },
        };

        let clear_values = &[color_clear_value, depth_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.data.render_pass.pass)
            .framebuffer(self.data.framebuffers[index].buffer)
            .render_area(render_area)
            .clear_values(clear_values);

        self.device.cmd_begin_render_pass(
            command_buffer.buffer,
            &info,
            vk::SubpassContents::SECONDARY_COMMAND_BUFFERS,
        );

        let secondary_command_buffers = (0..count)
            .map(|i| self.update_secondary_command_buffer(index, i))
            .collect::<Result<Vec<_>, _>>()?;

        // get the secondary command buffers
        let bf = secondary_command_buffers
            .iter()
            .map(|b| b.buffer)
            .collect::<Vec<_>>();

        // execute the command buffer
        self.device
            .cmd_execute_commands(command_buffer.buffer, &bf[..]);

        // end the render pass
        self.device.cmd_end_render_pass(command_buffer.buffer);

        // end the command buffer
        self.device.end_command_buffer(command_buffer.buffer)?;

        Ok(())
    }

    /// Updates a secondary command buffer for the app.
    // #[rustfmt::skip]
    unsafe fn update_secondary_command_buffer(
        &mut self,
        image_index: usize,
        model_index: usize,
    ) -> Result<CommandBuffer> {
        // Allocate

        let command_buffers = &mut self.data.secondary_command_buffers[image_index];
        while model_index >= command_buffers.len() {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(self.data.command_pools[image_index].pool)
                .level(vk::CommandBufferLevel::SECONDARY)
                .command_buffer_count(1);

            let command_buffer = self.device.allocate_command_buffers(&allocate_info)?[0];
            command_buffers.push(CommandBuffer::new(command_buffer));
        }

        let command_buffer = command_buffers[model_index];

        // Model

        let y = (((model_index % 2) as f32) * 2.5) - 1.25;
        let z = (((model_index / 2) as f32) * -2.0) + 1.0;

        let time = self.start.elapsed().as_secs_f32();

        let model = Mat4::from_translation(vec3(0.0, y, z))
            * Mat4::from_axis_angle(vec3(0.0, 0.0, 1.0), Deg(90.0) * time);

        let model_bytes =
            &*slice_from_raw_parts(&model as *const Mat4 as *const u8, size_of::<Mat4>());

        let opacity = (model_index + 1) as f32 * 0.25;
        let opacity_bytes = &opacity.to_ne_bytes()[..];

        // Commands

        let inheritance_info = vk::CommandBufferInheritanceInfo::builder()
            .render_pass(self.data.render_pass.pass)
            .subpass(0)
            .framebuffer(self.data.framebuffers[image_index].buffer);

        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE)
            .inheritance_info(&inheritance_info);

        self.device
            .begin_command_buffer(command_buffer.buffer, &info)?;

        self.device.cmd_bind_pipeline(
            command_buffer.buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.data.pipeline,
        );
        self.device.cmd_bind_vertex_buffers(
            command_buffer.buffer,
            0,
            &[self.data.vertex_buffer.buffer],
            &[0],
        );
        self.device.cmd_bind_index_buffer(
            command_buffer.buffer,
            self.data.index_buffer.buffer,
            0,
            vk::IndexType::UINT32,
        );
        self.device.cmd_bind_descriptor_sets(
            command_buffer.buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.data.pipeline_layout,
            0,
            &[self.data.descriptor_sets[image_index].set],
            &[],
        );
        self.device.cmd_push_constants(
            command_buffer.buffer,
            self.data.pipeline_layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            model_bytes,
        );
        self.device.cmd_push_constants(
            command_buffer.buffer,
            self.data.pipeline_layout,
            vk::ShaderStageFlags::FRAGMENT,
            64,
            opacity_bytes,
        );
        self.device.cmd_draw_indexed(
            command_buffer.buffer,
            self.data.indices.len() as u32,
            1,
            0,
            0,
            0,
        );

        self.device.end_command_buffer(command_buffer.buffer)?;

        Ok(command_buffer)
    }

    /// Updates the uniform buffer object for the app.
    unsafe fn update_uniform_buffer(&self, image_index: usize) -> Result<()> {
        // MVP

        let view = Mat4::look_at_rh(
            point3::<f32>(6.0, 0.0, 2.0),
            point3::<f32>(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
        );

        #[rustfmt::skip]
        let correction = Mat4::new(
            1.0,  0.0,       0.0, 0.0,
            0.0, -1.0,       0.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 1.0,
        );

        let proj = correction
            * cgmath::perspective(
                Deg(45.0),
                self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
                0.1,
                10.0,
            );

        let ubo = UniformBufferObject { view, proj };

        // Copy

        let memory = self.device.map_memory(
            self.data.uniform_buffers[image_index].memory,
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(&ubo, memory.cast(), 1);

        self.device
            .unmap_memory(self.data.uniform_buffers[image_index].memory);

        Ok(())
    }

    /// Recreates the swapchain for the app.
    // #[rustfmt::skip]
    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        self.device.device_wait_idle()?;
        self.destroy_swapchain();
        create_swapchain(
            window,
            &self.instance,
            &self.surface,
            &self.physical,
            &self.device,
            &mut self.data,
        )?;
        create_swapchain_views(&self.device, &mut self.data)?;
        create_render_pass(
            &self.instance,
            &self.physical,
            &self.samples,
            &self.device,
            &mut self.data,
        )?;
        create_pipeline(&self.device, &self.samples, &mut self.data)?;
        create_color_objects(
            &self.instance,
            &self.physical,
            &self.samples,
            &self.device,
            &mut self.data,
        )?;
        create_depth_objects(
            &self.instance,
            &self.physical,
            &self.samples,
            &self.device,
            &mut self.data,
        )?;
        create_framebuffers(&self.device, &mut self.data)?;
        create_uniform_buffers(&self.instance, &self.physical, &self.device, &mut self.data)?;
        create_descriptor_pool(&self.device, &mut self.data)?;
        create_descriptor_sets(&self.device, &mut self.data)?;
        create_command_buffers(&self.device, &mut self.data)?;
        self.data
            .textures_in_flight
            .resize(self.data.swapchain_images.len(), vk::Fence::null());
        Ok(())
    }

    /// Destroys the app.
    // #[rustfmt::skip]
    pub unsafe fn destroy(&mut self) {
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain();

        self.data
            .in_flight_fences
            .iter()
            .for_each(|f| self.device.destroy_fence(*f, None));
        self.data
            .render_finished_semaphores
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data
            .textures_available_semaphores
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data
            .command_pools
            .iter()
            .for_each(|p| p.destroy(&self.device));

        // destroy index buffer
        self.data.index_buffer.destroy(&self.device);

        // destroy vertex buffer
        self.data.vertex_buffer.destroy(&self.device);

        // destroy material sampler
        self.data.material_texture_sampler.destroy(&self.device);

        // destroy material texture
        self.data.material_texture.destroy(&self.device);

        // destroy material texture view
        self.data.material_texture_view.destroy(&self.device);

        // destroy command pool
        self.data.command_pool.destroy(&self.device);
        //
        self.device
            .destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);

        // destroy device
        self.device.destroy_device(None);

        // destroy surface
        self.instance.destroy_surface_khr(self.surface, None);

        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        // destroy instance
        self.instance.destroy_instance(None);
    }

    /// Destroys the parts of the app related to the swapchain.
    // #[rustfmt::skip]
    unsafe fn destroy_swapchain(&mut self) {
        // destroy descriptor pool
        self.device
            .destroy_descriptor_pool(self.data.descriptor_pool.pool, None);

        // destroy uniform buffers
        for i in 0..self.data.uniform_buffers.len() {
            self.data.uniform_buffers[i].destroy(&self.device);
        }

        // destroy depth texture
        self.data.depth_texture_view.destroy(&self.device);
        self.data.depth_texture.destroy(&self.device);

        // destroy color texture
        self.data.color_texture_view.destroy(&self.device);
        self.data.color_texture.destroy(&self.device);

        // destroy framebuffers
        self.data
            .framebuffers
            .iter()
            .for_each(|f| f.destroy(&self.device));

        // destroy pipeline
        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device
            .destroy_pipeline_layout(self.data.pipeline_layout, None);

        // destroy render pass
        self.data.render_pass.destroy(&self.device);

        // destroy swapchain views
        self.data
            .swapchain_image_views
            .iter()
            .for_each(|v| v.destroy(&self.device));

        // destroy swapchain
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
    }
}

/// The Vulkan handles and associated properties used by the app.
#[derive(Clone, Debug, Default)]
struct GraphicsDeviceData {
    // debug
    messenger: vk::DebugUtilsMessengerEXT,

    // physical Device / logical Device
    graphics_queue: Queue,
    present_queue: Queue,

    // swapchain
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<Texture>,
    swapchain_image_views: Vec<TextureView>,

    // pipeline
    render_pass: RenderPass,
    descriptor_set_layout: vk::DescriptorSetLayout,

    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,

    // framebuffers
    framebuffers: Vec<FrameBuffer>,

    // command Pool
    command_pool: CommandPool,

    // color texture
    color_texture: Texture,
    color_texture_view: TextureView,

    // depth texture
    depth_texture: Texture,
    depth_texture_view: TextureView,

    // material textures
    mip_levels: u32,
    material_texture: Texture,
    material_texture_view: TextureView,
    material_texture_sampler: Sampler,

    // model
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    // buffers
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    uniform_buffers: Vec<Buffer>,

    // descriptors
    descriptor_pool: DescriptorPool,
    descriptor_sets: Vec<DescriptorSet>,

    // command Buffers
    command_pools: Vec<CommandPool>,
    primary_command_buffers: Vec<CommandBuffer>,
    secondary_command_buffers: Vec<Vec<CommandBuffer>>,

    // sync objects
    textures_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    textures_in_flight: Vec<vk::Fence>,
}

unsafe fn create_instance(
    window: &Window,
    entry: &Entry,
    data: &mut GraphicsDeviceData,
) -> Result<Instance> {
    // Application Info

    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"D E I M O S\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    // Layers

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

    // Extensions

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216.
    let flags = if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        info!("Enabling extensions for macOS portability.");
        extensions.push(
            vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION
                .name
                .as_ptr(),
        );
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    if VALIDATION_ENABLED {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    // Create

    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .user_callback(Some(debug_callback));

    if VALIDATION_ENABLED {
        info = info.push_next(&mut debug_info);
    }

    let instance = entry.create_instance(&info, None)?;

    // Messenger

    if VALIDATION_ENABLED {
        data.messenger = instance.create_debug_utils_messenger_ext(&debug_info, None)?;
    }

    Ok(instance)
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("({:?}) {}", type_, message);
    } else {
        trace!("({:?}) {}", type_, message);
    }

    vk::FALSE
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SuitabilityError(pub &'static str);

unsafe fn pick_physical_device(
    instance: &Instance,
    surface: &vk::SurfaceKHR,
    data: &mut GraphicsDeviceData,
) -> Result<vk::PhysicalDevice> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, surface, physical_device) {
            warn!(
                "Skipping physical device (`{}`): {}",
                properties.device_name, error
            );
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            return Ok(physical_device);
        }
    }

    Err(anyhow!("Failed to find suitable physical device."))
}

unsafe fn check_physical_device(
    instance: &Instance,
    surface: &vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    QueueFamilyIndices::get(instance, surface, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, surface, physical_device)?;
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")));
    }

    let features = instance.get_physical_device_features(physical_device);
    if features.sampler_anisotropy != vk::TRUE {
        return Err(anyhow!(SuitabilityError("No sampler anisotropy.")));
    }

    Ok(())
}

unsafe fn check_physical_device_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    let extensions = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();
    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError(
            "Missing required device extensions."
        )))
    }
}

unsafe fn get_max_msaa_samples(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
) -> vk::SampleCountFlags {
    let properties = instance.get_physical_device_properties(*physical);
    let counts = properties.limits.framebuffer_color_sample_counts
        & properties.limits.framebuffer_depth_sample_counts;
    [
        vk::SampleCountFlags::_64,
        vk::SampleCountFlags::_32,
        vk::SampleCountFlags::_16,
        vk::SampleCountFlags::_8,
        vk::SampleCountFlags::_4,
        vk::SampleCountFlags::_2,
    ]
    .iter()
    .cloned()
    .find(|c| counts.contains(*c))
    .unwrap_or(vk::SampleCountFlags::_1)
}

unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    surface: &vk::SurfaceKHR,
    physical: &vk::PhysicalDevice,
    data: &mut GraphicsDeviceData,
) -> Result<Device> {
    // Queue Create Infos

    let indices = QueueFamilyIndices::get(instance, surface, *physical)?;

    let mut unique_indices = HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|i| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();

    // Layers

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    // Extensions

    let mut extensions = DEVICE_EXTENSIONS
        .iter()
        .map(|n| n.as_ptr())
        .collect::<Vec<_>>();

    // Required by Vulkan SDK on macOS since 1.3.216.
    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    // Features

    let features = vk::PhysicalDeviceFeatures::builder()
        .sampler_anisotropy(true)
        .sample_rate_shading(true);

    // Create

    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(*physical, &info, None)?;

    // Queues

    data.graphics_queue = Queue::create(device.get_device_queue(indices.graphics, 0));
    data.present_queue = Queue::create(device.get_device_queue(indices.present, 0));

    Ok(device)
}

unsafe fn create_swapchain(
    window: &Window,
    instance: &Instance,
    surface: &vk::SurfaceKHR,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // Image

    let indices = QueueFamilyIndices::get(instance, surface, *physical)?;
    let support = SwapchainSupport::get(instance, surface, *physical)?;

    let surface_format = get_swapchain_surface_format(&support.formats);
    let present_mode = get_swapchain_present_mode(&support.present_modes);
    let extent = get_swapchain_extent(window, support.capabilities);

    data.swapchain_format = surface_format.format;
    data.swapchain_extent = extent;

    let mut image_count = support.capabilities.min_image_count + 1;
    if support.capabilities.max_image_count != 0
        && image_count > support.capabilities.max_image_count
    {
        image_count = support.capabilities.max_image_count;
    }

    let mut queue_family_indices = vec![];
    let image_sharing_mode = if indices.graphics != indices.present {
        queue_family_indices.push(indices.graphics);
        queue_family_indices.push(indices.present);
        vk::SharingMode::CONCURRENT
    } else {
        vk::SharingMode::EXCLUSIVE
    };

    // Create

    let info = vk::SwapchainCreateInfoKHR::builder()
        .surface(*surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(image_sharing_mode)
        .queue_family_indices(&queue_family_indices)
        .pre_transform(support.capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(vk::SwapchainKHR::null());

    // create swap chain
    data.swapchain = device.create_swapchain_khr(&info, None)?;

    // data.swapchain_images = device.get_swapchain_images_khr(data.swapchain)?;

    // get swap chain images
    let images = device.get_swapchain_images_khr(data.swapchain)?;

    // map into textures
    data.swapchain_images = images
        .iter()
        .map(|i| Texture::create(*i, vk::DeviceMemory::null()))
        .collect::<Vec<_>>();
    Ok(())
}

fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    formats
        .iter()
        .cloned()
        .find(|f| {
            f.format == vk::Format::B8G8R8A8_SRGB
                && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or_else(|| formats[0])
}

fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    present_modes
        .iter()
        .cloned()
        .find(|m| *m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

fn get_swapchain_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::max_value() {
        capabilities.current_extent
    } else {
        let size = window.inner_size();
        let clamp = |min: u32, max: u32, v: u32| min.max(max.min(v));
        vk::Extent2D::builder()
            .width(clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
                size.width,
            ))
            .height(clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
                size.height,
            ))
            .build()
    }
}

unsafe fn create_swapchain_views(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    data.swapchain_image_views = data
        .swapchain_images
        .iter()
        .map(|i| {
            i.create_view(
                device,
                data.swapchain_format,
                vk::ImageAspectFlags::COLOR,
                1,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

unsafe fn create_render_pass(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    samples: &vk::SampleCountFlags,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // Attachments

    let color_attachment = vk::AttachmentDescription::builder()
        .format(data.swapchain_format)
        .samples(*samples)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let depth_stencil_attachment = vk::AttachmentDescription::builder()
        .format(get_depth_format(instance, physical, data)?)
        .samples(*samples)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::DONT_CARE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

    let color_resolve_attachment = vk::AttachmentDescription::builder()
        .format(data.swapchain_format)
        .samples(vk::SampleCountFlags::_1)
        .load_op(vk::AttachmentLoadOp::DONT_CARE)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

    // Subpasses

    let color_attachment_ref = vk::AttachmentReference::builder()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let depth_stencil_attachment_ref = vk::AttachmentReference::builder()
        .attachment(1)
        .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

    let color_resolve_attachment_ref = vk::AttachmentReference::builder()
        .attachment(2)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let color_attachments = &[color_attachment_ref];
    let resolve_attachments = &[color_resolve_attachment_ref];
    let subpass = vk::SubpassDescription::builder()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(color_attachments)
        .depth_stencil_attachment(&depth_stencil_attachment_ref)
        .resolve_attachments(resolve_attachments);

    // Dependencies

    let dependency = vk::SubpassDependency::builder()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
        )
        .src_access_mask(vk::AccessFlags::empty())
        .dst_stage_mask(
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
        )
        .dst_access_mask(
            vk::AccessFlags::COLOR_ATTACHMENT_WRITE
                | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
        );

    // Create

    let attachments = &[
        color_attachment,
        depth_stencil_attachment,
        color_resolve_attachment,
    ];
    let subpasses = &[subpass];
    let dependencies = &[dependency];
    let info = vk::RenderPassCreateInfo::builder()
        .attachments(attachments)
        .subpasses(subpasses)
        .dependencies(dependencies);

    data.render_pass = RenderPass::create(device.create_render_pass(&info, None)?);

    Ok(())
}

unsafe fn create_descriptor_set_layout(
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX);

    let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(1)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT);

    let bindings = &[ubo_binding, sampler_binding];
    let info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(bindings);

    data.descriptor_set_layout = device.create_descriptor_set_layout(&info, None)?;

    Ok(())
}

unsafe fn create_pipeline(
    device: &Device,
    samples: &vk::SampleCountFlags,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // Stages

    let vert = include_bytes!("../../shaders/vert.spv");
    let frag = include_bytes!("../../shaders/frag.spv");

    let vertex_shader = Shader::create(device, &vert[..])?;
    let fragment_shader = Shader::create(device, &frag[..])?;

    let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::VERTEX)
        .module(vertex_shader.module)
        .name(b"main\0");

    let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::FRAGMENT)
        .module(fragment_shader.module)
        .name(b"main\0");

    // Vertex Input State

    let binding_descriptions = &[Vertex::binding_description()];
    let attribute_descriptions = Vertex::attribute_descriptions();
    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(binding_descriptions)
        .vertex_attribute_descriptions(&attribute_descriptions);

    // Input Assembly State

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    // Viewport State

    let viewport = vk::Viewport::builder()
        .x(0.0)
        .y(0.0)
        .width(data.swapchain_extent.width as f32)
        .height(data.swapchain_extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0);

    let scissor = vk::Rect2D::builder()
        .offset(vk::Offset2D { x: 0, y: 0 })
        .extent(data.swapchain_extent);

    let viewports = &[viewport];
    let scissors = &[scissor];
    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(viewports)
        .scissors(scissors);

    // Rasterization State

    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .depth_bias_enable(false);

    // Multisample State

    let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(true)
        .min_sample_shading(0.2)
        .rasterization_samples(*samples);

    // Depth Stencil State

    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vk::CompareOp::LESS)
        .depth_bounds_test_enable(false)
        .stencil_test_enable(false);

    // Color Blend State

    let attachment = vk::PipelineColorBlendAttachmentState::builder()
        .color_write_mask(vk::ColorComponentFlags::all())
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
        .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
        .alpha_blend_op(vk::BlendOp::ADD);

    let attachments = &[attachment];
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(vk::LogicOp::COPY)
        .attachments(attachments)
        .blend_constants([0.0, 0.0, 0.0, 0.0]);

    // Push Constant Ranges

    let vert_push_constant_range = vk::PushConstantRange::builder()
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .offset(0)
        .size(64);

    let frag_push_constant_range = vk::PushConstantRange::builder()
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        .offset(64)
        .size(4);

    // Layout

    let set_layouts = &[data.descriptor_set_layout];
    let push_constant_ranges = &[vert_push_constant_range, frag_push_constant_range];
    let layout_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(set_layouts)
        .push_constant_ranges(push_constant_ranges);

    data.pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;

    // Create

    let stages = &[vert_stage, frag_stage];
    let info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(stages)
        .vertex_input_state(&vertex_input_state)
        .input_assembly_state(&input_assembly_state)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization_state)
        .multisample_state(&multisample_state)
        .depth_stencil_state(&depth_stencil_state)
        .color_blend_state(&color_blend_state)
        .layout(data.pipeline_layout)
        .render_pass(data.render_pass.pass)
        .subpass(0);

    data.pipeline = device
        .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)?
        .0[0];

    // clean up
    vertex_shader.destroy(&device);
    fragment_shader.destroy(&device);

    Ok(())
}

unsafe fn create_framebuffers(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    data.framebuffers = data
        .swapchain_image_views
        .iter()
        .map(|i| {
            let attachments = &[
                data.color_texture_view.view,
                data.depth_texture_view.view,
                i.view,
            ];
            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(data.render_pass.pass)
                .attachments(attachments)
                .width(data.swapchain_extent.width)
                .height(data.swapchain_extent.height)
                .layers(1);
            FrameBuffer::create(
                device
                    .create_framebuffer(&create_info, None)
                    .expect("Failed to create framebuffer."),
            )
        })
        .collect();

    Ok(())
}

unsafe fn create_command_pools(
    instance: &Instance,
    surface: &vk::SurfaceKHR,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // Global

    data.command_pool = create_command_pool(instance, surface, physical, device, data)?;

    // Per-framebuffer

    let num_images = data.swapchain_images.len();
    for _ in 0..num_images {
        let command_pool = create_command_pool(instance, surface, physical, device, data)?;
        data.command_pools.push(command_pool);
    }

    Ok(())
}

unsafe fn create_command_pool(
    instance: &Instance,
    surface: &vk::SurfaceKHR,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<CommandPool> {
    let indices = QueueFamilyIndices::get(instance, surface, *physical)?;

    let info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::TRANSIENT)
        .queue_family_index(indices.graphics);

    Ok(CommandPool::new(
        device.create_command_pool(&info, None)?,
    ))
}

unsafe fn create_color_objects(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    samples: &vk::SampleCountFlags,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // texture
    data.color_texture = create_texture(
        instance,
        physical,
        device,
        data,
        data.swapchain_extent.width,
        data.swapchain_extent.height,
        1,
        *samples,
        data.swapchain_format,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSIENT_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // texture view
    data.color_texture_view = data.color_texture.create_view(
        device,
        data.swapchain_format,
        vk::ImageAspectFlags::COLOR,
        1,
    )?;

    // all went fine
    Ok(())
}

unsafe fn create_depth_objects(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    samples: &vk::SampleCountFlags,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // get depth format
    let format = get_depth_format(instance, physical, data)?;

    // create depth texture
    data.depth_texture = create_texture(
        instance,
        physical,
        device,
        data,
        data.swapchain_extent.width,
        data.swapchain_extent.height,
        1,
        *samples,
        format,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // depth texture view
    data.depth_texture_view =
        data.depth_texture
            .create_view(device, format, vk::ImageAspectFlags::DEPTH, 1)?;

    // all went fine
    Ok(())
}

unsafe fn get_depth_format(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    data: &GraphicsDeviceData,
) -> Result<vk::Format> {
    let candidates = &[
        vk::Format::D32_SFLOAT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
    ];

    get_supported_format(
        instance,
        physical,
        data,
        candidates,
        vk::ImageTiling::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}

unsafe fn get_supported_format(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    data: &GraphicsDeviceData,
    candidates: &[vk::Format],
    tiling: vk::ImageTiling,
    features: vk::FormatFeatureFlags,
) -> Result<vk::Format> {
    candidates
        .iter()
        .cloned()
        .find(|f| {
            let properties = instance.get_physical_device_format_properties(*physical, *f);
            match tiling {
                vk::ImageTiling::LINEAR => properties.linear_tiling_features.contains(features),
                vk::ImageTiling::OPTIMAL => properties.optimal_tiling_features.contains(features),
                _ => false,
            }
        })
        .ok_or_else(|| anyhow!("Failed to find supported format!"))
}

unsafe fn create_texture_image(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // Load

    let image = File::open("/Users/prime/depot/github/deimos/resources/viking_room.png")?;

    let decoder = png::Decoder::new(image);
    let mut reader = decoder.read_info()?;

    let mut pixels = vec![0; reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let size = reader.info().raw_bytes() as u64;
    let (width, height) = reader.info().size();
    data.mip_levels = (width.max(height) as f32).log2().floor() as u32 + 1;

    if width != 1024 || height != 1024 || reader.info().color_type != png::ColorType::Rgba {
        panic!("Invalid texture image.");
    }

    // create staging
    let staging_buffer = Buffer::create(
        instance,
        physical,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    // write
    staging_buffer.write(device, 0, size, &pixels);

    // create material texture
    data.material_texture = create_texture(
        instance,
        physical,
        device,
        data,
        width,
        height,
        data.mip_levels,
        vk::SampleCountFlags::_1,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::SAMPLED
            | vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // Transition + Copy (image)
    transition_image_layout(
        device,
        data,
        data.material_texture.image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        data.mip_levels,
    )?;

    copy_buffer_to_image(
        device,
        data,
        staging_buffer.buffer,
        data.material_texture.image,
        width,
        height,
    )?;

    // destroy staging
    staging_buffer.destroy(&device);

    // generate
    generate_mipmaps(
        instance,
        physical,
        device,
        data,
        data.material_texture.image,
        vk::Format::R8G8B8A8_SRGB,
        width,
        height,
        data.mip_levels,
    )?;

    Ok(())
}

unsafe fn generate_mipmaps(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &GraphicsDeviceData,
    image: vk::Image,
    format: vk::Format,
    width: u32,
    height: u32,
    mip_levels: u32,
) -> Result<()> {
    // Support

    if !instance
        .get_physical_device_format_properties(*physical, format)
        .optimal_tiling_features
        .contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR)
    {
        return Err(anyhow!(
            "Texture image format does not support linear blitting!"
        ));
    }

    // Mipmaps

    let command_buffer = begin_single_time_commands(device, data)?;

    let subresource = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_array_layer(0)
        .layer_count(1)
        .level_count(1);

    let mut barrier = vk::ImageMemoryBarrier::builder()
        .image(image)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .subresource_range(subresource);

    let mut mip_width = width;
    let mut mip_height = height;

    for i in 1..mip_levels {
        barrier.subresource_range.base_mip_level = i - 1;
        barrier.old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
        barrier.new_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
        barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
        barrier.dst_access_mask = vk::AccessFlags::TRANSFER_READ;

        device.cmd_pipeline_barrier(
            command_buffer.buffer,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[] as &[vk::MemoryBarrier],
            &[] as &[vk::BufferMemoryBarrier],
            &[barrier],
        );

        let src_subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .mip_level(i - 1)
            .base_array_layer(0)
            .layer_count(1);

        let dst_subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .mip_level(i)
            .base_array_layer(0)
            .layer_count(1);

        let blit = vk::ImageBlit::builder()
            .src_offsets([
                vk::Offset3D { x: 0, y: 0, z: 0 },
                vk::Offset3D {
                    x: mip_width as i32,
                    y: mip_height as i32,
                    z: 1,
                },
            ])
            .src_subresource(src_subresource)
            .dst_offsets([
                vk::Offset3D { x: 0, y: 0, z: 0 },
                vk::Offset3D {
                    x: (if mip_width > 1 { mip_width / 2 } else { 1 }) as i32,
                    y: (if mip_height > 1 { mip_height / 2 } else { 1 }) as i32,
                    z: 1,
                },
            ])
            .dst_subresource(dst_subresource);

        device.cmd_blit_image(
            command_buffer.buffer,
            image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[blit],
            vk::Filter::LINEAR,
        );

        barrier.old_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
        barrier.new_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
        barrier.src_access_mask = vk::AccessFlags::TRANSFER_READ;
        barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;

        device.cmd_pipeline_barrier(
            command_buffer.buffer,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::DependencyFlags::empty(),
            &[] as &[vk::MemoryBarrier],
            &[] as &[vk::BufferMemoryBarrier],
            &[barrier],
        );

        if mip_width > 1 {
            mip_width /= 2;
        }

        if mip_height > 1 {
            mip_height /= 2;
        }
    }

    barrier.subresource_range.base_mip_level = mip_levels - 1;
    barrier.old_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
    barrier.new_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
    barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
    barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;

    device.cmd_pipeline_barrier(
        command_buffer.buffer,
        vk::PipelineStageFlags::TRANSFER,
        vk::PipelineStageFlags::FRAGMENT_SHADER,
        vk::DependencyFlags::empty(),
        &[] as &[vk::MemoryBarrier],
        &[] as &[vk::BufferMemoryBarrier],
        &[barrier],
    );

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}

unsafe fn create_texture_image_view(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    data.material_texture_view = data.material_texture.create_view(
        device,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageAspectFlags::COLOR,
        data.mip_levels,
    )?;

    // all went fine
    Ok(())
}

unsafe fn create_texture_sampler(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    // create sampler info
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
        .max_lod(data.mip_levels as f32)
        .mip_lod_bias(0.0);

    // create sampler
    data.material_texture_sampler = Sampler::create(device.create_sampler(&info, None)?);

    // all went fine
    Ok(())
}

fn load_model(data: &mut GraphicsDeviceData) -> Result<()> {
    // create the reader to use
    let mut reader = BufReader::new(File::open(
        "/Users/prime/depot/github/deimos/resources/viking_room.obj",
    )?);

    // load the object model
    let (models, _) = tobj::load_obj_buf(
        &mut reader,
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
        |_| Ok(Default::default()),
    )?;

    // get vertices
    let mut unique_vertices = HashMap::new();

    // loop over models and each indices
    for model in &models {
        for index in &model.mesh.indices {
            // calulate offsets
            let position_offset = (3 * index) as usize;
            let texel_offset = (2 * index) as usize;

            // create vertex
            let vertex = Vertex {
                position: vec3(
                    model.mesh.positions[position_offset],
                    model.mesh.positions[position_offset + 1],
                    model.mesh.positions[position_offset + 2],
                ),
                texel: vec2(
                    model.mesh.texcoords[texel_offset],
                    1.0 - model.mesh.texcoords[texel_offset + 1],
                ),
                color: vec3(1.0, 1.0, 1.0),
            };

            // push the indieces
            if let Some(index) = unique_vertices.get(&vertex) {
                data.indices.push(*index as u32);
            } else {
                let index = data.vertices.len();
                unique_vertices.insert(vertex, index);
                data.vertices.push(vertex);
                data.indices.push(index as u32);
            }
        }
    }

    Ok(())
}

unsafe fn create_vertex_buffer(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // Create (staging)

    let size = (size_of::<Vertex>() * data.vertices.len()) as u64;

    let staging_buffer = Buffer::create(
        instance,
        physical,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    // write
    staging_buffer.write(device, 0, size, &data.vertices);

    // create vertex buffer
    data.vertex_buffer = Buffer::create(
        instance,
        physical,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // copy buffer
    copy_buffer(device, data, staging_buffer, data.vertex_buffer, size)?;

    // clean
    staging_buffer.destroy(&device);

    Ok(())
}

unsafe fn create_index_buffer(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // size
    let size = (size_of::<u32>() * data.indices.len()) as u64;

    // create staging buffer
    let staging_buffer = Buffer::create(
        instance,
        physical,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    // write
    staging_buffer.write(device, 0, size, &data.indices);

    // create index buffer
    data.index_buffer = Buffer::create(
        instance,
        physical,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // copy
    copy_buffer(device, data, staging_buffer, data.index_buffer, size)?;

    // cleanup
    staging_buffer.destroy(&device);

    Ok(())
}

unsafe fn create_uniform_buffers(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &mut GraphicsDeviceData,
) -> Result<()> {
    // clear buffers
    data.uniform_buffers.clear();

    // create ne buffer for each swapchain image
    for _ in 0..data.swapchain_images.len() {
        let uniform_buffer = Buffer::create(
            instance,
            physical,
            device,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        // push the buffer
        data.uniform_buffers.push(uniform_buffer);
    }

    // all went fine
    Ok(())
}

unsafe fn create_descriptor_pool(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    let ubo_size = vk::DescriptorPoolSize::builder()
        .type_(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(data.swapchain_images.len() as u32);

    let sampler_size = vk::DescriptorPoolSize::builder()
        .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(data.swapchain_images.len() as u32);

    let pool_sizes = &[ubo_size, sampler_size];
    let info = vk::DescriptorPoolCreateInfo::builder()
        .pool_sizes(pool_sizes)
        .max_sets(data.swapchain_images.len() as u32);

    data.descriptor_pool = DescriptorPool::create(device.create_descriptor_pool(&info, None)?);

    Ok(())
}

unsafe fn create_descriptor_sets(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    // Allocate

    let layouts = vec![data.descriptor_set_layout; data.swapchain_images.len()];
    let info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(data.descriptor_pool.pool)
        .set_layouts(&layouts);

    // get sets
    let sets = device.allocate_descriptor_sets(&info)?;

    // get
    data.descriptor_sets = sets
        .iter()
        .map(|set| DescriptorSet::create(*set))
        .collect::<Vec<_>>();

    // Update

    for i in 0..data.swapchain_images.len() {
        let info = vk::DescriptorBufferInfo::builder()
            .buffer(data.uniform_buffers[i].buffer)
            .offset(0)
            .range(size_of::<UniformBufferObject>() as u64);

        let buffer_info = &[info];
        let ubo_write = vk::WriteDescriptorSet::builder()
            .dst_set(data.descriptor_sets[i].set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(buffer_info);

        let info = vk::DescriptorImageInfo::builder()
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view(data.material_texture_view.view)
            .sampler(data.material_texture_sampler.sampler);

        let image_info = &[info];
        let sampler_write = vk::WriteDescriptorSet::builder()
            .dst_set(data.descriptor_sets[i].set)
            .dst_binding(1)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(image_info);

        device.update_descriptor_sets(&[ubo_write, sampler_write], &[] as &[vk::CopyDescriptorSet]);
    }

    Ok(())
}

unsafe fn create_command_buffers(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    let num_images = data.swapchain_images.len();
    for image_index in 0..num_images {
        let allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(data.command_pools[image_index].pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffer = device.allocate_command_buffers(&allocate_info)?[0];
        data.primary_command_buffers
            .push(CommandBuffer::new(command_buffer));
    }

    data.secondary_command_buffers = vec![vec![]; data.swapchain_images.len()];

    Ok(())
}

unsafe fn create_sync_objects(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.textures_available_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
        data.render_finished_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);

        data.in_flight_fences
            .push(device.create_fence(&fence_info, None)?);
    }

    data.textures_in_flight = data
        .swapchain_images
        .iter()
        .map(|_| vk::Fence::null())
        .collect();

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
    present: u32,
}

impl QueueFamilyIndices {
    unsafe fn get(
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let mut present = None;
        for (index, properties) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(
                physical_device,
                index as u32,
                *surface,
            )? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError(
                "Missing required queue families."
            )))
        }
    }
}

#[derive(Clone, Debug)]
struct SwapchainSupport {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupport {
    unsafe fn get(
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        Ok(Self {
            capabilities: instance
                .get_physical_device_surface_capabilities_khr(physical_device, *surface)?,
            formats: instance.get_physical_device_surface_formats_khr(physical_device, *surface)?,
            present_modes: instance
                .get_physical_device_surface_present_modes_khr(physical_device, *surface)?,
        })
    }
}

unsafe fn copy_buffer(
    device: &Device,
    data: &GraphicsDeviceData,
    source: Buffer,
    destination: Buffer,
    size: vk::DeviceSize,
) -> Result<()> {
    let command_buffer = begin_single_time_commands(device, data)?;

    let regions = vk::BufferCopy::builder().size(size);
    device.cmd_copy_buffer(
        command_buffer.buffer,
        source.buffer,
        destination.buffer,
        &[regions],
    );

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}

unsafe fn create_texture(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    device: &Device,
    data: &GraphicsDeviceData,
    width: u32,
    height: u32,
    mip_levels: u32,
    samples: vk::SampleCountFlags,
    format: vk::Format,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> Result<Texture> {
    // create the image info using specified data
    let info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::_2D)
        .extent(vk::Extent3D {
            width,
            height,
            depth: 1,
        })
        .mip_levels(mip_levels)
        .array_layers(1)
        .format(format)
        .tiling(tiling)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .samples(samples);

    // create the actual image
    let image = device.create_image(&info, None)?;

    // get the requirements for the image
    let requirements = device.get_image_memory_requirements(image);

    // create the memory info using the requirements
    let info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            physical,
            data,
            properties,
            requirements,
        )?);

    // allocate the memory for the image
    let memory = device.allocate_memory(&info, None)?;

    // bind the memory to the image
    device.bind_image_memory(image, memory, 0)?;

    // all done create the texture
    Ok(Texture::create(image, memory))
}

unsafe fn transition_image_layout(
    device: &Device,
    data: &GraphicsDeviceData,
    image: vk::Image,
    format: vk::Format,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
    mip_levels: u32,
) -> Result<()> {
    let (src_access_mask, dst_access_mask, src_stage_mask, dst_stage_mask) =
        match (old_layout, new_layout) {
            (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
                vk::AccessFlags::empty(),
                vk::AccessFlags::TRANSFER_WRITE,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
            ),
            (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => (
                vk::AccessFlags::TRANSFER_WRITE,
                vk::AccessFlags::SHADER_READ,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
            ),
            _ => return Err(anyhow!("Unsupported image layout transition!")),
        };

    let command_buffer = begin_single_time_commands(device, data)?;

    let subresource = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(mip_levels)
        .base_array_layer(0)
        .layer_count(1);

    let barrier = vk::ImageMemoryBarrier::builder()
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(subresource)
        .src_access_mask(src_access_mask)
        .dst_access_mask(dst_access_mask);

    device.cmd_pipeline_barrier(
        command_buffer.buffer,
        src_stage_mask,
        dst_stage_mask,
        vk::DependencyFlags::empty(),
        &[] as &[vk::MemoryBarrier],
        &[] as &[vk::BufferMemoryBarrier],
        &[barrier],
    );

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}

unsafe fn copy_buffer_to_image(
    device: &Device,
    data: &GraphicsDeviceData,
    buffer: vk::Buffer,
    image: vk::Image,
    width: u32,
    height: u32,
) -> Result<()> {
    let command_buffer = begin_single_time_commands(device, data)?;

    let subresource = vk::ImageSubresourceLayers::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1);

    let region = vk::BufferImageCopy::builder()
        .buffer_offset(0)
        .buffer_row_length(0)
        .buffer_image_height(0)
        .image_subresource(subresource)
        .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
        .image_extent(vk::Extent3D {
            width,
            height,
            depth: 1,
        });

    device.cmd_copy_buffer_to_image(
        command_buffer.buffer,
        buffer,
        image,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        &[region],
    );

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}

unsafe fn get_memory_type_index(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
    data: &GraphicsDeviceData,
    properties: vk::MemoryPropertyFlags,
    requirements: vk::MemoryRequirements,
) -> Result<u32> {
    let memory = instance.get_physical_device_memory_properties(*physical);
    (0..memory.memory_type_count)
        .find(|i| {
            let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];
            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
}

unsafe fn begin_single_time_commands(
    device: &Device,
    data: &GraphicsDeviceData,
) -> Result<CommandBuffer> {
    // Allocate

    let info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_pool(data.command_pool.pool)
        .command_buffer_count(1);

    let command_buffer = device.allocate_command_buffers(&info)?[0];

    // Begin

    let info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    device.begin_command_buffer(command_buffer, &info)?;

    Ok(CommandBuffer::new(command_buffer))
}

unsafe fn end_single_time_commands(
    device: &Device,
    data: &GraphicsDeviceData,
    command_buffer: CommandBuffer,
) -> Result<()> {
    // End

    device.end_command_buffer(command_buffer.buffer)?;

    // Submit

    let command_buffers = &[command_buffer.buffer];
    let info = vk::SubmitInfo::builder().command_buffers(command_buffers);

    device.queue_submit(data.graphics_queue.queue, &[info], vk::Fence::null())?;
    device.queue_wait_idle(data.graphics_queue.queue)?;

    // Cleanup

    device.free_command_buffers(data.command_pool.pool, &[command_buffer.buffer]);

    Ok(())
}
