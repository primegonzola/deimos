// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use log::*;
use thiserror::Error;

use anyhow::{anyhow, Result};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::KhrSurfaceExtension;

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32,
}

impl QueueFamilyIndices {
    pub fn get(
        instance: &vulkanalia::Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Self> {
        unsafe {
            let properties = instance.get_physical_device_queue_family_properties(physical_device);

            let graphics = properties
                .iter()
                .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
                .map(|i| i as u32);

            let mut present = None;
            for (index, _properties) in properties.iter().enumerate() {
                if instance.get_physical_device_surface_support_khr(
                    physical_device,
                    index as u32,
                    surface,
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
}

#[derive(Clone, Debug)]
pub struct SwapchainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupport {
    pub fn get(
        instance: &vulkanalia::Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Self> {
        unsafe {
            Ok(Self {
                capabilities: instance
                    .get_physical_device_surface_capabilities_khr(physical_device, surface)?,
                formats: instance
                    .get_physical_device_surface_formats_khr(physical_device, surface)?,
                present_modes: instance
                    .get_physical_device_surface_present_modes_khr(physical_device, surface)?,
            })
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    pub model: cgmath::Matrix4<f32>,
    pub view: cgmath::Matrix4<f32>,
    pub projection: cgmath::Matrix4<f32>,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SuitabilityError(pub &'static str);
