// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use anyhow::Ok;
use anyhow::{anyhow, Result};
use log::*;
use std::collections::HashSet;
use std::ffi::CStr;
use std::fs::File;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSwapchainExtension;
use winit::window::Window;

use crate::vulkan::{QueueFamilyIndices, SuitabilityError, SwapchainSupport};

/// Whether the validation layers should be enabled.
pub const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
/// The name of the validation layers.
pub const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

/// The required device extensions.
pub const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
pub const PORTABILITY_MACOS_VERSION: vulkanalia::Version = vulkanalia::Version::new(1, 3, 216);

/// The maximum number of frames that can be processed concurrently.
pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub const MAX_DESCRIPTOR_SETS: usize = 10;

#[derive(Clone)]
pub struct VulkanApi {
    pub entry: vulkanalia::Entry,
    pub instance: vulkanalia::Instance,
    pub logical_device: vulkanalia::Device,
    pub physical_device: vk::PhysicalDevice,
}

impl VulkanApi {
    pub fn get_entry() -> Result<vulkanalia::Entry> {
        unsafe {
            let loader = vulkanalia::loader::LibloadingLoader::new(vulkanalia::loader::LIBRARY)?;
            let entry = vulkanalia::Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
            Ok(entry)
        }
    }

    pub fn get_memory_type_index(
        &self,
        properties: vk::MemoryPropertyFlags,
        requirements: vk::MemoryRequirements,
    ) -> Result<u32> {
        unsafe {
            let memory = self
                .instance
                .get_physical_device_memory_properties(self.physical_device);
            (0..memory.memory_type_count)
                .find(|i| {
                    let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
                    let memory_type = memory.memory_types[*i as usize];
                    suitable && memory_type.property_flags.contains(properties)
                })
                .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
        }
    }

    pub fn create_image_view(
        logical_device: &vulkanalia::Device,
        image: vk::Image,
        format: vk::Format,
        aspects: vk::ImageAspectFlags,
        mip_levels: u32,
    ) -> Result<vk::ImageView> {
        unsafe {
            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(aspects)
                .base_mip_level(0)
                .level_count(mip_levels)
                .base_array_layer(0)
                .layer_count(1);

            let info = vk::ImageViewCreateInfo::builder()
                .image(image)
                .view_type(vk::ImageViewType::_2D)
                .format(format)
                .subresource_range(subresource_range);

            Ok(logical_device.create_image_view(&info, None)?)
        }
    }

    pub fn create_image(
        &self,
        width: u32,
        height: u32,
        mip_levels: u32,
        samples: vk::SampleCountFlags,
        format: vk::Format,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<(vk::Image, vk::DeviceMemory)> {
        // Image

        unsafe {
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

            let image = self.logical_device.create_image(&info, None)?;

            // Memory

            let requirements = self.logical_device.get_image_memory_requirements(image);

            let info = vk::MemoryAllocateInfo::builder()
                .allocation_size(requirements.size)
                .memory_type_index(self.get_memory_type_index(properties, requirements)?);

            let image_memory = self.logical_device.allocate_memory(&info, None)?;

            self.logical_device
                .bind_image_memory(image, image_memory, 0)?;

            Ok((image, image_memory))
        }
    }

    pub fn get_depth_format(&self) -> Result<vk::Format> {
        let candidates = &[
            vk::Format::D32_SFLOAT,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::Format::D24_UNORM_S8_UINT,
        ];

        self.get_supported_format(
            candidates,
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        )
    }

    pub fn get_supported_format(
        &self,
        candidates: &[vk::Format],
        tiling: vk::ImageTiling,
        features: vk::FormatFeatureFlags,
    ) -> Result<vk::Format> {
        unsafe {
            candidates
                .iter()
                .cloned()
                .find(|f| {
                    let properties = self
                        .instance
                        .get_physical_device_format_properties(self.physical_device, *f);
                    match tiling {
                        vk::ImageTiling::LINEAR => {
                            properties.linear_tiling_features.contains(features)
                        }
                        vk::ImageTiling::OPTIMAL => {
                            properties.optimal_tiling_features.contains(features)
                        }
                        _ => false,
                    }
                })
                .ok_or_else(|| anyhow!("Failed to find supported format!"))
        }
    }

    pub fn get_max_msaa_samples(
        instance: &vulkanalia::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::SampleCountFlags {
        unsafe {
            let properties = instance.get_physical_device_properties(physical_device);
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
    }

    pub fn create_surface(
        instance: &vulkanalia::Instance,
        display: &Window,
        window: &Window,
    ) -> Result<vk::SurfaceKHR> {
        unsafe {
            Ok(vulkanalia::window::create_surface(
                &instance, &display, &window,
            )?)
        }
    }

    pub fn create_instance(
        entry: &Entry,
        window: &Window,
        title: &str,
    ) -> Result<(vulkanalia::Instance, Option<vk::DebugUtilsMessengerEXT>)> {
        unsafe {
            let application_info = vk::ApplicationInfo::builder()
                .application_name(title.as_bytes())
                .application_version(vk::make_version(1, 0, 0))
                .engine_name(b"No Engine\0")
                .engine_version(vk::make_version(1, 0, 0))
                .api_version(vk::make_version(1, 0, 0));

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
            let mut extensions = vulkanalia::window::get_required_instance_extensions(window)
                .iter()
                .map(|e| e.as_ptr())
                .collect::<Vec<_>>();

            // Required by Vulkan SDK on macOS since 1.3.216.
            let flags =
                if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
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

            // the instance
            let instance = entry.create_instance(&info, None)?;

            // Messenger
            let mut messenger = None;
            if VALIDATION_ENABLED {
                messenger = Some(instance.create_debug_utils_messenger_ext(&debug_info, None)?);
            }

            // all went ok
            Ok((instance, messenger))
        }
    }

    pub fn select_physical_device(
        instance: &Instance,
        surface: vk::SurfaceKHR,
    ) -> Result<(vk::PhysicalDevice, vk::SampleCountFlags)> {
        unsafe {
            for physical_device in instance.enumerate_physical_devices()? {
                let properties = instance.get_physical_device_properties(physical_device);

                if let Err(error) = Self::check_physical_device(instance, physical_device, surface)
                {
                    warn!(
                        "Skipping physical device (`{}`): {}",
                        properties.device_name, error
                    );
                } else {
                    info!("Selected physical device (`{}`).", properties.device_name);
                    return Ok((
                        physical_device,
                        VulkanApi::get_max_msaa_samples(instance, physical_device),
                    ));
                }
            }

            Err(anyhow!("Failed to find suitable physical device."))
        }
    }

    pub fn check_physical_device(
        instance: &vulkanalia::Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<()> {
        unsafe {
            QueueFamilyIndices::get(&instance, physical_device, surface)?;
            Self::check_physical_device_extensions(instance, physical_device)?;

            let support = SwapchainSupport::get(&instance, physical_device, surface)?;
            if support.formats.is_empty() || support.present_modes.is_empty() {
                return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")));
            }

            let features = instance.get_physical_device_features(physical_device);
            if features.sampler_anisotropy != vk::TRUE {
                return Err(anyhow!(SuitabilityError("No sampler anisotropy.")));
            }

            Ok(())
        }
    }

    unsafe fn check_physical_device_extensions(
        instance: &vulkanalia::Instance,
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

    pub fn create_logical_device(
        entry: vulkanalia::Entry,
        instance: vulkanalia::Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<(vulkanalia::Device, vk::Queue, vk::Queue)> {
        // Queue Create Infos
        unsafe {
            let indices = QueueFamilyIndices::get(&instance, physical_device, surface)?;

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

            let device = instance.create_device(physical_device, &info, None)?;

            // Queues

            let graphics_queue = device.get_device_queue(indices.graphics, 0);
            let present_queue = device.get_device_queue(indices.present, 0);

            Ok((device, graphics_queue, present_queue))
        }
    }

    pub fn get_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        formats
            .iter()
            .cloned()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or_else(|| formats[0])
    }

    pub fn get_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        present_modes
            .iter()
            .cloned()
            .find(|m| *m == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub fn get_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
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

    pub fn create_swapchain(
        &self,
        window: &Window,
        surface: vk::SurfaceKHR,
    ) -> Result<(
        vk::SwapchainKHR,
        vk::Format,
        vk::Extent2D,
        Vec<vk::Image>,
        Vec<vk::ImageView>,
    )> {
        unsafe {
            let indices = QueueFamilyIndices::get(&self.instance, self.physical_device, surface)?;
            let support = SwapchainSupport::get(&self.instance, self.physical_device, surface)?;

            let surface_format = VulkanApi::get_surface_format(&support.formats);
            let present_mode = VulkanApi::get_present_mode(&support.present_modes);
            let extent = VulkanApi::get_extent(window, support.capabilities);

            let format = surface_format.format;
            let extent = extent;

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
                .surface(surface)
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

            let swapchain = self.logical_device.create_swapchain_khr(&info, None)?;

            // Images
            let images = self.logical_device.get_swapchain_images_khr(swapchain)?;

            let views = images
                .iter()
                .map(|i| {
                    VulkanApi::create_image_view(
                        &self.logical_device,
                        *i,
                        format,
                        vk::ImageAspectFlags::COLOR,
                        1,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok((swapchain, format, extent, images, views))
        }
    }

    pub fn create_command_pool(&self, surface: vk::SurfaceKHR) -> Result<vk::CommandPool> {
        unsafe {
            let indices = QueueFamilyIndices::get(&self.instance, self.physical_device, surface)?;
            let info = vk::CommandPoolCreateInfo::builder()
                .queue_family_index(indices.graphics)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
            let command_pool = self.logical_device.create_command_pool(&info, None)?;
            Ok(command_pool)
        }
    }

    pub fn create_shader_module(
        device: &vulkanalia::Device,
        bytecode: &[u8],
    ) -> Result<vk::ShaderModule> {
        unsafe {
            let bytecode = vulkanalia::bytecode::Bytecode::new(bytecode).unwrap();
            let info = vk::ShaderModuleCreateInfo::builder()
                .code_size(bytecode.code_size())
                .code(bytecode.code());
            Ok(device.create_shader_module(&info, None)?)
        }
    }

    pub fn begin_single_commands(
        &self,
        command_pool: vk::CommandPool,
    ) -> Result<vk::CommandBuffer> {
        unsafe {
            // Allocate
            let info = vk::CommandBufferAllocateInfo::builder()
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_pool(command_pool)
                .command_buffer_count(1);
            let command_buffer = self.logical_device.allocate_command_buffers(&info)?[0];
            // Begin
            let info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            self.logical_device
                .begin_command_buffer(command_buffer, &info)?;
            Ok(command_buffer)
        }
    }

    pub fn end_single_commands(
        &self,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
        command_buffer: vk::CommandBuffer,
    ) -> Result<()> {
        unsafe {
            // End
            self.logical_device.end_command_buffer(command_buffer)?;
            // Submit
            let command_buffers = &[command_buffer];
            let info = vk::SubmitInfo::builder().command_buffers(command_buffers);
            self.logical_device
                .queue_submit(queue, &[info], vk::Fence::null())?;
            self.logical_device.queue_wait_idle(queue)?;
            // Cleanup
            self.logical_device
                .free_command_buffers(command_pool, &[command_buffer]);
            Ok(())
        }
    }

    pub fn copy_buffer(
        &self,
        command_pool: vk::CommandPool,
        queue: vk::Queue,
        source: vk::Buffer,
        destination: vk::Buffer,
        size: vk::DeviceSize,
    ) -> Result<()> {
        unsafe {
            let command_buffer = self.begin_single_commands(command_pool)?;
            let regions = vk::BufferCopy::builder().size(size);
            self.logical_device
                .cmd_copy_buffer(command_buffer, source, destination, &[regions]);
            self.end_single_commands(queue, command_pool, command_buffer)?;
            Ok(())
        }
    }

    pub fn copy_buffer_to_image(
        &self,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
        buffer: vk::Buffer,
        image: vk::Image,
        width: u32,
        height: u32,
    ) -> Result<()> {
        unsafe {
            let command_buffer = self.begin_single_commands(command_pool)?;
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
            self.logical_device.cmd_copy_buffer_to_image(
                command_buffer,
                buffer,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[region],
            );
            self.end_single_commands(queue, command_pool, command_buffer)?;
            Ok(())
        }
    }

    pub fn transition_image_layout(
        &self,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
        image: vk::Image,
        _format: vk::Format,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
        mip_levels: u32,
    ) -> Result<()> {
        unsafe {
            let (src_access_mask, dst_access_mask, src_stage_mask, dst_stage_mask) =
                match (old_layout, new_layout) {
                    (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
                        vk::AccessFlags::empty(),
                        vk::AccessFlags::TRANSFER_WRITE,
                        vk::PipelineStageFlags::TOP_OF_PIPE,
                        vk::PipelineStageFlags::TRANSFER,
                    ),
                    (
                        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    ) => (
                        vk::AccessFlags::TRANSFER_WRITE,
                        vk::AccessFlags::SHADER_READ,
                        vk::PipelineStageFlags::TRANSFER,
                        vk::PipelineStageFlags::FRAGMENT_SHADER,
                    ),
                    _ => return Err(anyhow!("Unsupported image layout transition!")),
                };

            let command_buffer = self.begin_single_commands(command_pool)?;
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

            self.logical_device.cmd_pipeline_barrier(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                vk::DependencyFlags::empty(),
                &[] as &[vk::MemoryBarrier],
                &[] as &[vk::BufferMemoryBarrier],
                &[barrier],
            );
            self.end_single_commands(queue, command_pool, command_buffer)?;
            Ok(())
        }
    }

    pub fn generate_mipmaps(
        &self,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
        image: vk::Image,
        format: vk::Format,
        width: u32,
        height: u32,
        mip_levels: u32,
    ) -> Result<()> {
        unsafe {
            // Support
            if !self
                .instance
                .get_physical_device_format_properties(self.physical_device, format)
                .optimal_tiling_features
                .contains(vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR)
            {
                return Err(anyhow!(
                    "Texture image format does not support linear blitting!"
                ));
            }

            // Mipmaps
            let command_buffer = self.begin_single_commands(command_pool)?;

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

                self.logical_device.cmd_pipeline_barrier(
                    command_buffer,
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

                self.logical_device.cmd_blit_image(
                    command_buffer,
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

                self.logical_device.cmd_pipeline_barrier(
                    command_buffer,
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

            self.logical_device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::DependencyFlags::empty(),
                &[] as &[vk::MemoryBarrier],
                &[] as &[vk::BufferMemoryBarrier],
                &[barrier],
            );
            self.end_single_commands(queue, command_pool, command_buffer)?;
            Ok(())
        }
    }

    // pub fn create_buffer(
    //     &self,
    //     size: vk::DeviceSize,
    //     usage: vk::BufferUsageFlags,
    //     properties: vk::MemoryPropertyFlags,
    // ) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    //     unsafe {
    //         // Buffer
    //         let buffer_info = vk::BufferCreateInfo::builder()
    //             .size(size)
    //             .usage(usage)
    //             .sharing_mode(vk::SharingMode::EXCLUSIVE);
    //         let buffer = self.logical_device.create_buffer(&buffer_info, None)?;
    //         // Memory
    //         let requirements = self.logical_device.get_buffer_memory_requirements(buffer);
    //         let memory_info = vk::MemoryAllocateInfo::builder()
    //             .allocation_size(requirements.size)
    //             .memory_type_index(self.get_memory_type_index(properties, requirements)?);
    //         let buffer_memory = self.logical_device.allocate_memory(&memory_info, None)?;
    //         self.logical_device
    //             .bind_buffer_memory(buffer, buffer_memory, 0)?;
    //         Ok((buffer, buffer_memory))
    //     }
    // }

    pub fn create_buffer<T>(
        &self,
        size: usize,
        usage: vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
    ) -> Result<(vk::Buffer, vk::DeviceMemory)> {
        unsafe {
            let fsize = (size_of::<T>() * size) as u64;
            // Buffer
            let buffer_info = vk::BufferCreateInfo::builder()
                .size(fsize)
                .usage(usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);
            let buffer = self.logical_device.create_buffer(&buffer_info, None)?;
            // Memory
            let requirements = self.logical_device.get_buffer_memory_requirements(buffer);
            let memory_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(requirements.size)
                .memory_type_index(self.get_memory_type_index(properties, requirements)?);
            let buffer_memory = self.logical_device.allocate_memory(&memory_info, None)?;
            self.logical_device
                .bind_buffer_memory(buffer, buffer_memory, 0)?;
            Ok((buffer, buffer_memory))
        }
    }

    pub fn write_buffer<T>(
        &self,
        buffer_memory: vk::DeviceMemory,
        offset: usize,
        data: &Vec<T>,
    ) -> Result<()> {
        unsafe {
            // Copy (staging)
            let memory = self.logical_device.map_memory(
                buffer_memory,
                (size_of::<T>() * offset) as u64,
                (size_of::<T>() * data.len()) as u64,
                vk::MemoryMapFlags::empty(),
            )?;
            memcpy(data.as_ptr(), memory.cast(), data.len());
            self.logical_device.unmap_memory(buffer_memory);
            Ok(())
        }
    }

    pub fn create_staged_buffer<T>(
        &self,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
        data: &Vec<T>,
        usage: vk::BufferUsageFlags,
    ) -> Result<(vk::Buffer, vk::DeviceMemory)> {
        unsafe {
            // Create (staging)
            let (staging_buffer, staging_buffer_memory) = self.create_buffer::<T>(
                data.len(),
                vk::BufferUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            )?;

            // write
            self.write_buffer(staging_buffer_memory, 0, data)?;

            // Create (vertex)
            let (target_buffer, target_memory) = self.create_buffer::<T>(
                data.len(),
                vk::BufferUsageFlags::TRANSFER_DST | usage,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )?;

            // Copy (vertex)
            self.copy_buffer(
                command_pool,
                queue,
                staging_buffer,
                target_buffer,
                (data.len() * size_of::<T>()) as u64,
            )?;

            // Cleanup
            self.logical_device.destroy_buffer(staging_buffer, None);
            self.logical_device.free_memory(staging_buffer_memory, None);

            Ok((target_buffer, target_memory))
        }
    }

    pub fn load_image(
        &self,
        queue: vk::Queue,
        command_pool: vk::CommandPool,
        path: &str,
    ) -> Result<(vk::Image, vk::DeviceMemory, u32)> {
        unsafe {
            let image = File::open(path)?;

            let decoder = png::Decoder::new(image);
            let mut reader = decoder.read_info()?;

            let mut pixels = vec![0; reader.info().raw_bytes()];
            reader.next_frame(&mut pixels)?;

            let size = reader.info().raw_bytes();
            let (width, height) = reader.info().size();
            let mip_levels = (width.max(height) as f32).log2().floor() as u32 + 1;

            if width != 1024 || height != 1024 || reader.info().color_type != png::ColorType::Rgba {
                panic!("Invalid texture image.");
            }

            // Create (staging)
            let (staging_buffer, staging_buffer_memory) = self.create_buffer::<u8>(
                size,
                vk::BufferUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            )?;

            // Copy (staging)
            self.write_buffer(staging_buffer_memory, 0, &pixels)?;

            // Create (image)
            let (handle, memory) = self.create_image(
                width,
                height,
                mip_levels,
                vk::SampleCountFlags::_1,
                vk::Format::R8G8B8A8_SRGB,
                vk::ImageTiling::OPTIMAL,
                vk::ImageUsageFlags::SAMPLED
                    | vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::TRANSFER_SRC,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )?;

            // Transition + Copy (image)
            self.transition_image_layout(
                queue,
                command_pool,
                handle,
                vk::Format::R8G8B8A8_SRGB,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                mip_levels,
            )?;

            self.copy_buffer_to_image(queue, command_pool, staging_buffer, handle, width, height)?;

            // Cleanup
            self.logical_device.destroy_buffer(staging_buffer, None);
            self.logical_device.free_memory(staging_buffer_memory, None);

            // Mipmaps
            self.generate_mipmaps(
                queue,
                command_pool,
                handle,
                vk::Format::R8G8B8A8_SRGB,
                width,
                height,
                mip_levels,
            )?;

            Ok((handle, memory, mip_levels))
        }
    }

    pub fn create_framebuffer(
        &self,
        render_pass: vk::RenderPass,
        attachments: &[vk::ImageView],
        extent: vk::Extent2D,
        layers: u32,
    ) -> Result<vk::Framebuffer> {
        unsafe {
            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(layers);
            Ok(self.logical_device.create_framebuffer(&create_info, None)?)
        }
    }

    pub fn create_render_pass(
        &self,
        format: vk::Format,
        samples: vk::SampleCountFlags,
    ) -> Result<vk::RenderPass> {
        unsafe {
            // Attachments
            let color_attachment = vk::AttachmentDescription::builder()
                .format(format)
                .samples(samples)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

            let depth_stencil_attachment = vk::AttachmentDescription::builder()
                .format(self.get_depth_format()?)
                .samples(samples)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::DONT_CARE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

            let color_resolve_attachment = vk::AttachmentDescription::builder()
                .format(format)
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

            let render_pass = self.logical_device.create_render_pass(&info, None)?;

            Ok(render_pass)
        }
    }

    pub fn allocate_primary_command_buffers(
        &self,
        command_pool: vk::CommandPool,
        count: u32,
    ) -> Result<Vec<vk::CommandBuffer>> {
        unsafe {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(count);
            Ok(self
                .logical_device
                .allocate_command_buffers(&allocate_info)?)
        }
    }

    pub fn reset_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        unsafe {
            self.logical_device.reset_command_buffer(
                command_buffer,
                vk::CommandBufferResetFlags::RELEASE_RESOURCES,
            )?;
            Ok(())
        }
    }

    pub fn begin_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        unsafe {
            let info = vk::CommandBufferBeginInfo::builder();
            self.logical_device
                .begin_command_buffer(command_buffer, &info)?;
            Ok(())
        }
    }

    pub fn bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline: vk::Pipeline,
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline,
            );
            Ok(())
        }
    }

    pub fn bind_descriptor_sets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        descriptor_sets: &[vk::DescriptorSet],
        dynamic_offsets: &[u32],
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_bind_descriptor_sets(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_sets,
                dynamic_offsets,
            );
            Ok(())
        }
    }

    pub fn bind_index_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        index_type: vk::IndexType,
    ) -> Result<()> {
        unsafe {
            self.logical_device
                .cmd_bind_index_buffer(command_buffer, buffer, offset, index_type);
            Ok(())
        }
    }

    pub fn draw(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_draw(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            );
            Ok(())
        }
    }

    pub fn draw_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_draw_indexed(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            );
            Ok(())
        }
    }

    pub fn bind_vertex_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        buffers: &[vk::Buffer],
        offsets: &[vk::DeviceSize],
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_bind_vertex_buffers(
                command_buffer,
                first_binding,
                buffers,
                offsets,
            );
            Ok(())
        }
    }

    pub fn draw_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_draw_indirect(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,
            );
            Ok(())
        }
    }

    pub fn draw_indexed_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_draw_indexed_indirect(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,
            );
            Ok(())
        }
    }

    pub fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        unsafe {
            self.logical_device.end_command_buffer(command_buffer)?;
            Ok(())
        }
    }

    pub fn end_render_pass(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        unsafe {
            // end render pass
            self.logical_device.cmd_end_render_pass(command_buffer);

            Ok(())
        }
    }

    pub fn begin_render_pass(
        &self,
        info: vk::RenderPassBeginInfoBuilder,
        command_buffer: vk::CommandBuffer,
    ) -> Result<()> {
        unsafe {
            self.logical_device.cmd_begin_render_pass(
                command_buffer,
                &info,
                vk::SubpassContents::INLINE,
            );
            Ok(())
        }
    }

    pub fn create_descriptor_set_layout(&self) -> Result<vk::DescriptorSetLayout> {
        unsafe {
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
            let layout = self
                .logical_device
                .create_descriptor_set_layout(&info, None)?;

            Ok(layout)
        }
    }

    pub fn create_descriptor_pool(&self, count: usize) -> Result<vk::DescriptorPool> {
        unsafe {
            let ubo_size = vk::DescriptorPoolSize::builder()
                .type_(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(count as u32);

            let sampler_size = vk::DescriptorPoolSize::builder()
                .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(count as u32);

            let pool_sizes = &[ubo_size, sampler_size];
            let info = vk::DescriptorPoolCreateInfo::builder()
                .pool_sizes(pool_sizes)
                .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
                .max_sets(count as u32);

            let descriptor_pool = self.logical_device.create_descriptor_pool(&info, None)?;

            Ok(descriptor_pool)
        }
    }

    pub fn create_albedo_sampler(&self, mip_levels: u32) -> Result<vk::Sampler> {
        unsafe {
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

            let albedo_sampler = self.logical_device.create_sampler(&info, None)?;

            Ok(albedo_sampler)
        }
    }

    pub fn create_pipeline_layout(
        &self,
        descriptor_set_layout: vk::DescriptorSetLayout,
    ) -> Result<vk::PipelineLayout> {
        unsafe {
            // Layout
            let set_layouts = &[descriptor_set_layout];
            let layout_info = vk::PipelineLayoutCreateInfo::builder().set_layouts(set_layouts);
            Ok(self
                .logical_device
                .create_pipeline_layout(&layout_info, None)?)
        }
    }

    pub fn create_pipeline(
        &self,
        render_pass: vk::RenderPass,
        pipeline_layout: vk::PipelineLayout,
        extent: vk::Extent2D,
        samples: vk::SampleCountFlags,
        binding_description: vk::VertexInputBindingDescription,
        attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,
    ) -> Result<vk::Pipeline> {
        unsafe {
            // Stages
            let vert = include_bytes!("../../shaders/compiled/blit_vs.spv");
            let frag = include_bytes!("../../shaders/compiled/blit_fs.spv");

            let vert_shader_module =
                VulkanApi::create_shader_module(&self.logical_device, &vert[..])?;
            let frag_shader_module =
                VulkanApi::create_shader_module(&self.logical_device, &frag[..])?;

            let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_shader_module)
                .name(b"main\0");
            let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_shader_module)
                .name(b"main\0");

            // Vertex Input State
            let binding_descriptions = &[binding_description];
            // let attribute_descriptions = Mesh::attribute_descriptions();
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
                .width(extent.width as f32)
                .height(extent.height as f32)
                .min_depth(0.0)
                .max_depth(1.0);

            let scissor = vk::Rect2D::builder()
                .offset(vk::Offset2D { x: 0, y: 0 })
                .extent(extent);

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
                .rasterization_samples(samples);

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
                .blend_enable(false);

            let attachments = &[attachment];
            let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
                .logic_op_enable(false)
                .logic_op(vk::LogicOp::COPY)
                .attachments(attachments)
                .blend_constants([0.0, 0.0, 0.0, 0.0]);

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
                .layout(pipeline_layout)
                .render_pass(render_pass)
                .subpass(0);

            let pipeline = self
                .logical_device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)?
                .0[0];

            // Cleanup
            self.logical_device
                .destroy_shader_module(vert_shader_module, None);
            self.logical_device
                .destroy_shader_module(frag_shader_module, None);

            Ok(pipeline)
        }
    }

    pub fn create_framebuffers(
        &self,
        render_pass: vk::RenderPass,
        extent: vk::Extent2D,
        attachment: vk::ImageView,
        depth: vk::ImageView,
        views: &Vec<vk::ImageView>,
    ) -> Result<Vec<vk::Framebuffer>> {
        let framebuffers = views
            .iter()
            .map(|i| self.create_framebuffer(render_pass, &[attachment, depth, *i], extent, 1))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(framebuffers)
    }

    pub fn create_descriptor_sets(
        &self,
        descriptor_pool: vk::DescriptorPool,
        descriptor_set_layout: vk::DescriptorSetLayout,
        count: usize,
    ) -> Result<Vec<vk::DescriptorSet>> {
        unsafe {
            // Allocate
            let layouts = vec![descriptor_set_layout; count];
            let info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(descriptor_pool)
                .set_layouts(&layouts);
            Ok(self.logical_device.allocate_descriptor_sets(&info)?)
        }
    }

    pub fn update_descriptor_sets<T>(
        &self,
        descriptor_sets: &Vec<vk::DescriptorSet>,
        uniform_buffers: &Vec<(vk::Buffer, vk::DeviceMemory)>,
        image_view: vk::ImageView,
        sampler: vk::Sampler,
        count: usize,
    ) -> Result<()> {
        unsafe {
            // Update
            for i in 0..count {
                let info = vk::DescriptorBufferInfo::builder()
                    .buffer(uniform_buffers[i].0)
                    .offset(0)
                    .range(size_of::<T>() as u64);

                let buffer_info = &[info];
                let ubo_write = vk::WriteDescriptorSet::builder()
                    .dst_set(descriptor_sets[i])
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(buffer_info);

                let info = vk::DescriptorImageInfo::builder()
                    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .image_view(image_view)
                    .sampler(sampler);

                let image_info = &[info];
                let sampler_write = vk::WriteDescriptorSet::builder()
                    .dst_set(descriptor_sets[i])
                    .dst_binding(1)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(image_info);

                self.logical_device.update_descriptor_sets(
                    &[ubo_write, sampler_write],
                    &[] as &[vk::CopyDescriptorSet],
                );
            }
            Ok(())
        }
    }

    pub fn create_uniform_buffers<T>(
        &self,
        count: usize,
    ) -> Result<Vec<(vk::Buffer, vk::DeviceMemory)>> {
        let mut uniform_buffers = Vec::new();
        for _ in 0..count {
            uniform_buffers.push(self.create_buffer::<T>(
                1,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            )?);
        }
        Ok(uniform_buffers)
    }

    pub fn create_command_buffers(
        &self,
        command_pool: vk::CommandPool,
        count: usize,
    ) -> Result<Vec<vk::CommandBuffer>> {
        unsafe {
            // Allocate
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(count as u32);

            // allocate and save
            let command_buffers = self
                .logical_device
                .allocate_command_buffers(&allocate_info)?;

            Ok(command_buffers)
        }
    }
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
