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
use std::ptr::slice_from_raw_parts;
use std::time::Instant;

use anyhow::{anyhow, Result};
use cgmath::{point3, vec2, vec3, Deg};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::Version;
use winit::window::Window;

use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;

use super::QueueFamilyIndices;
use super::SuitabilityError;
use super::SwapChainSupport;

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

pub struct Device {
    entry: Entry,
    instance: vulkanalia::Instance,
    surface: vk::SurfaceKHR,
    physical: vk::PhysicalDevice,
    device: vulkanalia::Device,
    samples: vk::SampleCountFlags,
    messenger: Option<vk::DebugUtilsMessengerEXT>,
    // messenger: vk::DebugUtilsMessengerEXT,
    // graphics_queue: Queue,
    // present_queue: Queue,
    // swapchain: Swapchain,
    // swapchain_textures: Vec<Texture>,
    // depth_texture: Texture,
    // render_pass: RenderPass,
    // descriptor_set_layout: DescriptorSetLayout,
    // pipeline_layout: PipelineLayout,
    // graphics_pipeline: Pipeline,
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
            let device = create_logical_device(&entry, &instance, &surface, &physical)?;

            // Queues
            // data.graphics_queue = Queue::create(device.get_device_queue(indices.graphics, 0));
            // data.present_queue = Queue::create(device.get_device_queue(indices.present, 0));

            // init app instance
            Ok(Self {
                entry,
                instance,
                surface,
                physical,
                device,
                samples,
                messenger,
            })
        }
    }

    pub fn destroy(&self) {
        unsafe {
            // wait until device is idle
            self.device.device_wait_idle().unwrap();

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
) -> Result<vulkanalia::Device> {
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

    Ok(device)
}

// unsafe fn create_swapchain_textures(device: &Device, data: &mut GraphicsDeviceData) -> Result<()> {
//     // get swap chain images
//     let images = device.get_swapchain_images_khr(data.swapchain.swapchain)?;

//     // map into textures
//     data.swapchain_textures = images
//         .iter()
//         .map(|i| Texture::create(*i, vk::DeviceMemory::null()))
//         .collect::<Vec<_>>();

//     Ok(())
// }
