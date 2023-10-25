// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

#[derive(Clone, Debug, Default)]
pub struct VulkanDeviceSync {
    pub in_flight_fence: vk::Fence,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub fences_in_flight: Vec<vk::Fence>,
    pub textures_in_flight: Vec<vk::Fence>,
}

impl VulkanDeviceSync {
    pub fn create(
        logical_device: &vulkanalia::Device,
        frame_count: usize,
        texture_count: usize,
    ) -> Result<Self> {
        unsafe {
            let mut data = Self::default();
            let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
            let semaphore_info = vk::SemaphoreCreateInfo::builder();
            for _ in 0..frame_count {
                data.image_available_semaphores
                    .push(logical_device.create_semaphore(&semaphore_info, None)?);
                data.render_finished_semaphores
                    .push(logical_device.create_semaphore(&semaphore_info, None)?);
                data.fences_in_flight
                    .push(logical_device.create_fence(&fence_info, None)?);
            }
            for _ in 0..texture_count {
                data.textures_in_flight.push(vk::Fence::null());
            }
            Ok(data)
        }
    }

    pub fn destroy(&self, logical_device: &vulkanalia::Device) {
        unsafe {
            self.fences_in_flight
                .iter()
                .for_each(|f| logical_device.destroy_fence(*f, None));
            self.render_finished_semaphores
                .iter()
                .for_each(|s| logical_device.destroy_semaphore(*s, None));
            self.image_available_semaphores
                .iter()
                .for_each(|s| logical_device.destroy_semaphore(*s, None));
        }
    }
}
