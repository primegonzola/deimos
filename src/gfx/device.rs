#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

use anyhow::{anyhow, Result};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::Version;
use winit::window::Window;

use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;

use super::{
    FrameBuffer, QueueFamilyIndices, SuitabilityError, SwapChainSupport, Texture, TextureView,
};

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

struct DeviceSyncData {
    textures_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    in_flight_textures: Vec<vk::Fence>,
}

struct DeviceTargetData {
    albedo_texture: Texture,
    albedo_texture_view: TextureView,
    depth_texture: Texture,
    depth_texture_view: TextureView,
}

struct SwapchainData {
    handle: vk::SwapchainKHR,
    extent: vk::Extent2D,
    format: vk::Format,
    framebuffers: Vec<FrameBuffer>,
    render_pass: vk::RenderPass,
    textures: Vec<Texture>,
    views: Vec<TextureView>,
    target: DeviceTargetData,
}

struct QueueData {
    graphics: vk::Queue,
    present: vk::Queue,
}

pub struct Device {
    entry: Entry,
    instance: vulkanalia::Instance,
    surface: vk::SurfaceKHR,
    physical: vk::PhysicalDevice,
    device: vulkanalia::Device,
    samples: vk::SampleCountFlags,
    messenger: Option<vk::DebugUtilsMessengerEXT>,
    swapchain: SwapchainData,
    queue: QueueData,
    sync: DeviceSyncData,
    frame: usize,
}

impl Device {
    pub fn create(window: &Window, title: &str) -> Result<Self> {
        unsafe {
            let loader = LibloadingLoader::new(LIBRARY)?;
            let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
            let (instance, messenger) = create_instance(&entry, window, title)?;
            let surface = vk_window::create_surface(&instance, &window, &window)?;
            let physical = pick_physical_device(&instance, &surface)?;
            let samples = get_max_msaa_samples(&instance, &physical);

            // create the logical device
            let (device, graphics_queue, present_queue) =
                create_logical_device(&entry, &instance, &surface, &physical)?;

            // create the swapchain
            let swapchain =
                construct_swapchain(window, &instance, &surface, &physical, &device, &samples)?;

            // create sync objects
            let sync = create_sync_objects(&device, &swapchain)?;

            // init app instance
            Ok(Self {
                entry,
                instance,
                surface,
                physical,
                device,
                samples,
                messenger,
                swapchain,
                queue: QueueData {
                    graphics: graphics_queue,
                    present: present_queue,
                },
                sync,
                frame: 0,
            })
        }
    }

    /// update the app.
    pub fn update(&mut self, window: &Window, count: usize) -> Result<()> {
        unsafe {
            // create an in flight fence to wait for
            let in_flight_fence = self.sync.in_flight_fences[self.frame];

            // wait for the fence
            self.device
                .wait_for_fences(&[in_flight_fence], true, u64::max_value())?;

            // get next image
            let result = self.device.acquire_next_image_khr(
                self.swapchain.handle,
                u64::max_value(),
                self.sync.textures_available_semaphores[self.frame],
                vk::Fence::null(),
            );

            // get the image or rebuild if not found
            let index = match result {
                Ok((index, _)) => index as usize,
                Err(vk::ErrorCode::OUT_OF_DATE_KHR) => {
                    return recontruct_swapchain(
                        window, instance, surface, physical, device, samples, swapchain,
                    )
                }
                Err(e) => return Err(anyhow!(e)),
            };

            // get the current image to use
            let texture_in_flight = self.sync.in_flight_textures[index];

            // check if valid
            if !texture_in_flight.is_null() {
                // wait for it until it is valid
                self.device
                    .wait_for_fences(&[texture_in_flight], true, u64::max_value())?;
            }

            // set next image to use
            self.sync.in_flight_textures[index] = in_flight_fence;

            // update command buffer
            // self.update_command_buffer(index, count)?;

            // update uniform buffer
            // self.update_uniform_buffer(index)?;

            let wait_semaphores = &[self.sync.textures_available_semaphores[self.frame]];
            let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let command_buffers = &[self.sync.primary_command_buffers[index].buffer];
            let signal_semaphores = &[self.sync.render_finished_semaphores[self.frame]];
            let submit_info = vk::SubmitInfo::builder()
                .wait_semaphores(wait_semaphores)
                .wait_dst_stage_mask(wait_stages)
                .command_buffers(command_buffers)
                .signal_semaphores(signal_semaphores);

            // reset all fences
            self.device.reset_fences(&[in_flight_fence])?;

            // submit buffers to queue
            self.device
                .queue_submit(self.queue.graphics, &[submit_info], in_flight_fence)?;

            // get the swapchain
            let swapchains = &[self.swapchain.handle];

            // image index to present
            let indices = &[index as u32];

            // get the present infoe
            let present_info = vk::PresentInfoKHR::builder()
                .wait_semaphores(signal_semaphores)
                .swapchains(swapchains)
                .image_indices(indices);

            // get the current presentation info
            let result = self
                .device
                .queue_present_khr(self.queue.present, &present_info);

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
    }

    pub fn destroy(&self) {
        unsafe {
            // wait until device is idle
            self.device.device_wait_idle().unwrap();

            // destroy fences
            self.sync
                .in_flight_fences
                .iter()
                .for_each(|f| self.device.destroy_fence(*f, None));
            self.sync
                .render_finished_semaphores
                .iter()
                .for_each(|s| self.device.destroy_semaphore(*s, None));
            self.sync
                .textures_available_semaphores
                .iter()
                .for_each(|s| self.device.destroy_semaphore(*s, None));

            // deconstruct swapchain
            destroy_swapchain(&self.device, &self.swapchain);

            // destroy device
            self.device.destroy_device(None);

            // destroy surface
            self.instance.destroy_surface_khr(self.surface, None);

            // check when validation is enabled
            if VALIDATION_ENABLED && self.messenger.is_some() {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.messenger.unwrap(), None);
            }

            // destroy instance
            self.instance.destroy_instance(None);
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

unsafe fn create_sync_objects(
    device: &vulkanalia::Device,
    swapchain: &SwapchainData,
) -> Result<DeviceSyncData> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    // create sync object
    let mut data = DeviceSyncData {
        textures_available_semaphores: vec![],
        render_finished_semaphores: vec![],
        in_flight_fences: vec![],
        in_flight_textures: vec![],
    };

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.textures_available_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
        data.render_finished_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);

        data.in_flight_fences
            .push(device.create_fence(&fence_info, None)?);
    }

    // get the the inflight texture fences
    data.in_flight_textures = swapchain
        .textures
        .iter()
        .map(|_| vk::Fence::null())
        .collect();

    Ok(data)
}

unsafe fn create_instance(
    entry: &Entry,
    window: &Window,
    title: &str,
) -> Result<(Instance, Option<vk::DebugUtilsMessengerEXT>)> {
    // Application Info

    let application_info = vk::ApplicationInfo::builder()
        .application_name(title.as_bytes())
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

    // Create info
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

    let mut messenger = None;
    // Messenger
    if VALIDATION_ENABLED {
        messenger = Some(instance.create_debug_utils_messenger_ext(&debug_info, None)?);
    }
    Ok((instance, messenger))
}

unsafe fn pick_physical_device(
    instance: &Instance,
    surface: &vk::SurfaceKHR,
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

    let support = SwapChainSupport::get(instance, surface, physical_device)?;
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
) -> Result<(vulkanalia::Device, vk::Queue, vk::Queue)> {
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
    let graphics_queue = device.get_device_queue(indices.graphics, 0);
    let present_queue = device.get_device_queue(indices.present, 0);

    Ok((device, graphics_queue, present_queue))
}

unsafe fn create_texture(
    instance: &vulkanalia::Instance,
    physical: &vk::PhysicalDevice,
    device: &vulkanalia::Device,
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

unsafe fn create_swapchain_albedo_objects(
    instance: &vulkanalia::Instance,
    physical: &vk::PhysicalDevice,
    device: &vulkanalia::Device,
    samples: &vk::SampleCountFlags,
    width: u32,
    height: u32,
    format: vk::Format,
) -> Result<(Texture, TextureView)> {
    // texture
    let texture = create_texture(
        instance,
        physical,
        device,
        width,
        height,
        1,
        *samples,
        format,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSIENT_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // texture view
    let view = texture.create_view(device, format, vk::ImageAspectFlags::COLOR, 1)?;

    // all went fine
    Ok((texture, view))
}

unsafe fn create_swapchain_depth_objects(
    instance: &vulkanalia::Instance,
    physical: &vk::PhysicalDevice,
    device: &vulkanalia::Device,
    samples: &vk::SampleCountFlags,
    width: u32,
    height: u32,
) -> Result<(Texture, TextureView)> {
    // get depth format
    let format = get_depth_format(instance, physical)?;

    // create depth texture
    let texture = create_texture(
        instance,
        physical,
        device,
        width,
        height,
        1,
        *samples,
        format,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // depth texture view
    let view = texture.create_view(device, format, vk::ImageAspectFlags::DEPTH, 1)?;

    // all went fine
    Ok((texture, view))
}

unsafe fn construct_swapchain(
    window: &Window,
    instance: &vulkanalia::Instance,
    surface: &vk::SurfaceKHR,
    physical: &vk::PhysicalDevice,
    device: &vulkanalia::Device,
    samples: &vk::SampleCountFlags,
) -> Result<SwapchainData> {
    // create swapchain
    let (swapchain, format, extent) =
        create_swapchain(window, instance, surface, physical, device)?;

    // get swap chain images
    let images = device.get_swapchain_images_khr(swapchain)?;

    // map into textures
    let textures = images
        .iter()
        .map(|i| Texture::create(*i, vk::DeviceMemory::null()))
        .collect::<Vec<_>>();

    // map into views
    let views = textures
        .iter()
        .map(|i| i.create_view(device, format, vk::ImageAspectFlags::COLOR, 1))
        .collect::<Result<Vec<_>, _>>()?;

    // create render pass
    let render_pass = create_render_pass(instance, physical, device, samples, format)?;

    // create albedo info
    let (albedo_texture, albedo_texture_view) = create_swapchain_albedo_objects(
        &instance,
        &physical,
        &device,
        &samples,
        extent.width,
        extent.height,
        format,
    )?;

    // create depth info
    let (depth_texture, depth_texture_view) = create_swapchain_depth_objects(
        &instance,
        &physical,
        &device,
        &samples,
        extent.width,
        extent.height,
    )?;

    // create framebuffers
    let framebuffers: Vec<_> = views
        .iter()
        .map(|i| {
            FrameBuffer::create(
                device,
                &render_pass,
                &[albedo_texture_view, depth_texture_view, *i],
                extent.width,
                extent.height,
            )
            .expect("Failed to create framebuffer.")
        })
        .collect();

    // create target
    let target = DeviceTargetData {
        albedo_texture,
        albedo_texture_view,
        depth_texture,
        depth_texture_view,
    };

    // all done
    Ok(SwapchainData {
        extent,
        handle: swapchain,
        format,
        framebuffers,
        render_pass,
        target,
        textures,
        views,
    })
}

unsafe fn recontruct_swapchain(
    window: &Window,
    instance: &vulkanalia::Instance,
    surface: &vk::SurfaceKHR,
    physical: &vk::PhysicalDevice,
    device: &vulkanalia::Device,
    samples: &vk::SampleCountFlags,
    swapchain: &SwapchainData,
) -> Result<SwapchainData> {
    // destrpy current swap chain
    destroy_swapchain(device, swapchain);

    // create new swap chain
    let swapchain = construct_swapchain(window, &instance, &surface, &physical, &device, &samples)?;

    // all done
    Ok(swapchain)
}

unsafe fn destroy_swapchain(device: &vulkanalia::Device, swapchain: &SwapchainData) {
    // destroy framebuffers
    swapchain
        .framebuffers
        .iter()
        .for_each(|f| f.destroy(&device));

    // destroy render pass
    device.destroy_render_pass(swapchain.render_pass, None);

    // destroy albedo texture & view
    swapchain.target.albedo_texture.destroy(&device);
    swapchain.target.albedo_texture_view.destroy(&device);

    // destroy depth texture & view
    swapchain.target.depth_texture.destroy(&device);
    swapchain.target.depth_texture_view.destroy(&device);

    // destroy swapchain views, textures not needed
    swapchain.views.iter().for_each(|v| v.destroy(&device));

    // destroy swapchain
    device.destroy_swapchain_khr(swapchain.handle, None);
}

unsafe fn create_swapchain(
    window: &Window,
    instance: &vulkanalia::Instance,
    surface: &vk::SurfaceKHR,
    physical: &vk::PhysicalDevice,
    device: &vulkanalia::Device,
) -> Result<(vk::SwapchainKHR, vk::Format, vk::Extent2D)> {
    let indices = QueueFamilyIndices::get(instance, surface, *physical)?;
    let support = SwapChainSupport::get(instance, surface, *physical)?;

    let surface_format = get_surface_format(&support.formats);
    let present_mode = get_present_mode(&support.present_modes);
    let extent = get_extent(window, support.capabilities);

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

    // build info
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
    let swapchain = device.create_swapchain_khr(&info, None)?;

    // all went fine
    Ok((swapchain, format, extent))
}

unsafe fn create_render_pass(
    instance: &vulkanalia::Instance,
    physical: &vk::PhysicalDevice,
    device: &vulkanalia::Device,
    samples: &vk::SampleCountFlags,
    format: vk::Format,
) -> Result<vk::RenderPass> {
    // Attachments
    let color_attachment = vk::AttachmentDescription::builder()
        .format(format)
        .samples(*samples)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let depth_stencil_attachment = vk::AttachmentDescription::builder()
        .format(get_depth_format(instance, physical)?)
        .samples(*samples)
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

    let render_pass = device.create_render_pass(&info, None)?;

    Ok(render_pass)
}

fn get_surface_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
    formats
        .iter()
        .cloned()
        .find(|f| {
            f.format == vk::Format::B8G8R8A8_SRGB
                && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or_else(|| formats[0])
}

fn get_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    present_modes
        .iter()
        .cloned()
        .find(|m| *m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

fn get_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
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

unsafe fn get_memory_type_index(
    instance: &vulkanalia::Instance,
    physical: &vk::PhysicalDevice,
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

unsafe fn get_depth_format(
    instance: &vulkanalia::Instance,
    physical: &vk::PhysicalDevice,
) -> Result<vk::Format> {
    let candidates = &[
        vk::Format::D32_SFLOAT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
    ];

    get_supported_format(
        instance,
        physical,
        candidates,
        vk::ImageTiling::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}

unsafe fn get_supported_format(
    instance: &Instance,
    physical: &vk::PhysicalDevice,
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
