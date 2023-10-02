// SPDX-License-Identifier: MIT

// #![allow(dead_code)]
use anyhow::Result;
use std::sync::{Arc, Mutex};
use vulkano::image::ImageAccess;
use vulkano::sync::GpuFuture;
use vulkano_win::VkSurfaceBuild;
use winit::dpi::PhysicalSize;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Device {
    pub device: Arc<vulkano::device::Device>,
    pub instance: Arc<vulkano::instance::Instance>,
    pub physical: Arc<vulkano::device::physical::PhysicalDevice>,
    pub queue: Arc<vulkano::device::Queue>,
    pub surface: Arc<vulkano::swapchain::Surface>,
    pub swapchain: Arc<vulkano::swapchain::Swapchain>,
    pub swapchain_images: Vec<Arc<vulkano::image::SwapchainImage>>,
    pub image_index: u32,
    pub framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>>,
    pub render_pass: Arc<vulkano::render_pass::RenderPass>,
    pub viewport: vulkano::pipeline::graphics::viewport::Viewport,
    pub previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
    pub suboptimal: bool,
    pub acquire_future: Option<vulkano::swapchain::SwapchainAcquireFuture>,
    pub memory_allocator: vulkano::memory::allocator::StandardMemoryAllocator,
    pub command_buffer_allocator:
        vulkano::command_buffer::allocator::StandardCommandBufferAllocator,
    pub data: Arc<Mutex<DeviceData>>,
}

impl Device {
    pub fn create(event_loop: &EventLoop<()>) -> Result<Device> {
        // create the vulkan library instance
        let library = vulkano::VulkanLibrary::new().unwrap();

        //
        // When we create an instance, we have to pass a list of extensions that we want to enable.
        // All the window-drawing functionalities are part of non-core extensions that we need to
        // enable manually. To do so, we ask the `vulkano_win` crate for the list of extensions
        // required to draw to a window.
        //
        let required_extensions = vulkano_win::required_extensions(&library);

        //
        // create the instance.
        //
        let instance = vulkano::instance::Instance::new(
            library,
            vulkano::instance::InstanceCreateInfo {
                enabled_extensions: required_extensions,
                // Enable enumerating devices that use
                // non-conformant Vulkan implementations. (e.g.MoltenVK on MacOS)
                enumerate_portability: true,
                ..Default::default()
            },
        )
        .unwrap();

        //
        // create the window
        //
        // This is done by creating a `WindowBuilder` from the `winit` crate, then calling the
        // `build_vk_surface` method provided by the `VkSurfaceBuild` trait from `vulkano_win`. If you
        // ever get an error about `build_vk_surface` being undefined in one of your projects, this
        // probably means that you forgot to import this trait.
        //
        // This returns a `vulkano::swapchain::Surface` object that contains both a cross-platform
        // winit window and a cross-platform Vulkan surface that represents the surface of the window.
        //
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        //
        // Choose device extensions that we're going to use. In order to present images to a surface,
        // we need a `Swapchain`, which is provided by the `khr_swapchain` extension.
        //
        let device_extensions = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::empty()
        };

        //
        // We then choose which physical device to use. First, we enumerate all the available physical
        // devices, then apply filters to narrow them down to those that can support our needs.
        //
        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| {
                //
                // Some devices may not support the extensions or features that your application, or
                // report properties and limits that are not sufficient for your application. These
                // should be filtered out here.
                //
                p.supported_extensions().contains(&device_extensions)
            })
            .filter_map(|p| {
                //
                // For each physical device, we try to find a suitable queue family that will execute
                // our draw commands.
                //
                // Devices can provide multiple queues to run commands in parallel (for example a draw
                // queue and a compute queue), similar to CPU threads. This is something you have to
                // have to manage manually in Vulkan. Queues of the same type belong to the same queue
                // family.
                //
                // Here, we look for a single queue family that is suitable for our purposes. In a
                // real-world application, you may want to use a separate dedicated transfer queue to
                // handle data transfers in parallel with graphics operations. You may also need a
                // separate queue for compute operations, if your application uses those.
                //
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        //
                        // We select a queue family that supports graphics operations. When drawing to
                        // a window surface, as we do in this example, we also need to check that
                        // queues in this queue family are capable of presenting images to the surface.
                        //
                        q.queue_flags
                            .intersects(vulkano::device::QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    //
                    // The code here searches for the first queue family that is suitable. If none is
                    // found, `None` is returned to `filter_map`, which disqualifies this physical
                    // device.
                    //
                    .map(|i| (p, i as u32))
            })
            //
            // All the physical devices that pass the filters above are suitable for the application.
            // However, not every device is equal, some are preferred over others. Now, we assign each
            // physical device a score, and pick the device with the lowest ("best") score.
            //
            // In this example, we simply select the best-scoring device to use in the application.
            // In a real-world setting, you may want to use the best-scoring device only as a "default"
            // or "recommended" device, and let the user choose the device themself.
            //
            .min_by_key(|(p, _)| {
                // We assign a lower score to device types that are likely to be faster/better.
                match p.properties().device_type {
                    vulkano::device::physical::PhysicalDeviceType::DiscreteGpu => 0,
                    vulkano::device::physical::PhysicalDeviceType::IntegratedGpu => 1,
                    vulkano::device::physical::PhysicalDeviceType::VirtualGpu => 2,
                    vulkano::device::physical::PhysicalDeviceType::Cpu => 3,
                    vulkano::device::physical::PhysicalDeviceType::Other => 4,
                    _ => 5,
                }
            })
            .expect("no suitable physical device found");

        // Some little debug infos.
        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        //
        // Now initializing the device. This is probably the most important object of Vulkan.
        // An iterator of created queues is returned by the function alongside the device.
        //
        let (device, mut queues) = vulkano::device::Device::new(
            // Which physical device to connect to.
            physical_device.clone(),
            vulkano::device::DeviceCreateInfo {
                //
                // A list of optional features and extensions that our program needs to work correctly.
                // Some parts of the Vulkan specs are optional and must be enabled manually at device
                // creation. In this example the only thing we are going to need is the `khr_swapchain`
                // extension that allows us to draw to a window.
                //
                enabled_extensions: device_extensions,

                //
                // The list of queues that we are going to use. Here we only use one queue, from the
                // previously chosen queue family.
                //
                queue_create_infos: vec![vulkano::device::QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],

                ..Default::default()
            },
        )
        .unwrap();

        //
        // Since we can request multiple queues, the `queues` variable is in fact an iterator. We only
        // use one queue in this example, so we just retrieve the first and only element of the iterator.
        //
        let queue = queues.next().unwrap();

        //
        // Before we can draw on the surface, we have to create what is called a swapchain. Creating a
        // swapchain allocates the color buffers that will contain the image that will ultimately be
        // visible on the screen. These images are returned alongside the swapchain.
        //
        let (swapchain, swapchain_images) = {
            //
            // Querying the capabilities of the surface. When we create the swapchain we can only pass
            // values that are allowed by the capabilities.
            //
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            //
            // Choosing the internal format that the images will have.
            //
            let image_format = Some(
                device
                    .physical_device()
                    .surface_formats(&surface, Default::default())
                    .unwrap()[0]
                    .0,
            );
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

            // Please take a look at the docs for the meaning of the parameters we didn't mention.
            vulkano::swapchain::Swapchain::new(
                device.clone(),
                surface.clone(),
                vulkano::swapchain::SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,

                    image_format,

                    // The dimensions of the window, only used to initially setup the swapchain.
                    //
                    // NOTE:
                    // On some drivers the swapchain dimensions are specified by
                    // `surface_capabilities.current_extent` and the swapchain size must use these
                    // dimensions. These dimensions are always the same as the window dimensions.
                    //
                    // However, other drivers don't specify a value, i.e.
                    // `surface_capabilities.current_extent` is `None`. These drivers will allow
                    // anything, but the only sensible value is the window dimensions.
                    //
                    // Both of these cases need the swapchain to use the window dimensions, so we just
                    // use that.
                    image_extent: window.inner_size().into(),

                    image_usage: vulkano::image::ImageUsage::COLOR_ATTACHMENT,

                    // The alpha mode indicates how the alpha value of the final image will behave. For
                    // example, you can choose whether the window will be opaque or transparent.
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),

                    ..Default::default()
                },
            )
            .unwrap()
        };

        //
        // The next step is to create a *render pass*, which is an object that describes where the
        // output of the graphics pipeline will go. It describes the layout of the images where the
        // colors, depth and/or stencil information will be written.
        //
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                // `color` is a custom name we give to the first and only attachment.
                color: {
                    // `load: Clear` means that we ask the GPU to clear the content of this attachment
                    // at the start of the drawing.
                    load: Clear,
                    // `store: Store` means that we ask the GPU to store the output of the draw in the
                    // actual image. We could also ask it to discard the result.
                    store: Store,
                    // `format: <ty>` indicates the type of the format of the image. This has to be one
                    // of the types of the `vulkano::format` module (or alternatively one of your
                    // structs that implements the `FormatDesc` trait). Here we use the same format as
                    // the swapchain.
                    format: swapchain.image_format(),
                    //
                    // `samples: 1` means that we ask the GPU to use one sample to determine the value
                    // of each pixel in the color attachment. We could use a larger value
                    // (multisampling) for antialiasing. An example of this can be found in
                    // msaa-renderpass.rs.
                    //
                    samples: 1,
                },
            },
            pass: {
                // We use the attachment named `color` as the one and only color attachment.
                color: [color],
                // No depth-stencil attachment is indicated with empty brackets.
                depth_stencil: {},
            },
        )
        .unwrap();
        //
        // Dynamic viewports allow us to recreate just the viewport when the window is resized.
        // Otherwise we would have to recreate the whole pipeline.
        //
        let viewport = vulkano::pipeline::graphics::viewport::Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };
        //
        // The render pass we created above only describes the layout of our framebuffers. Before we
        // can draw we also need to create the actual framebuffers.
        //
        // Since we need to draw to multiple images, we are going to create a different framebuffer for
        // each image.
        let (viewport, framebuffers) =
            window_size_dependent_setup(&swapchain_images, render_pass.clone(), &viewport);

        //
        // In some situations, the swapchain will become invalid by itself. This includes for example
        // when the window is resized (as the images of the swapchain will no longer match the
        // window's) or, on Android, when the application went to the background and goes back to the
        // foreground.
        //
        // In this situation, acquiring a swapchain image or presenting it will return an error.
        // Rendering to an image of that swapchain will not produce any error, but may or may not work.
        // To continue rendering, we need to recreate the swapchain by creating a new swapchain. Here,
        // we remember that we need to do this for the next loop iteration.
        //
        let recreate_swapchain = false;

        //
        // In the loop below we are going to submit commands to the GPU. Submitting a command produces
        // an object that implements the `GpuFuture` trait, which holds the resources for as long as
        // they are in use by the GPU.
        //
        // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to avoid
        // that, we store the submission of the previous frame here.
        //
        let previous_frame_end = Some(vulkano::sync::future::now(device.clone()).boxed());

        // create memory allocator for the device
        let memory_allocator =
            vulkano::memory::allocator::StandardMemoryAllocator::new_default(device.clone());

        // Before we can start creating and recording command buffers, we need a way of allocating
        // them. Vulkano provides a command buffer allocator, which manages raw Vulkan command pools
        // underneath and provides a safe interface for them.
        let command_buffer_allocator =
            vulkano::command_buffer::allocator::StandardCommandBufferAllocator::new(
                device.clone(),
                Default::default(),
            );

        // all finaly done
        Ok(Self {
            device: device.clone(),
            instance: instance.clone(),
            physical: physical_device.clone(),
            queue: queue.clone(),
            surface: surface.clone(),
            swapchain: swapchain.clone(),
            swapchain_images: swapchain_images,
            image_index: 0,
            framebuffers: framebuffers,
            render_pass: render_pass.clone(),
            viewport: viewport,
            previous_frame_end: previous_frame_end,
            suboptimal: false,
            acquire_future: None,
            memory_allocator: memory_allocator,
            command_buffer_allocator: command_buffer_allocator,
            data: Arc::new(Mutex::new(DeviceData {
                recreate_swapchain: recreate_swapchain,
            })),
        })
    }

    pub fn dimensions(&self) -> PhysicalSize<u32> {
        let window = self
            .surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap();
        window.inner_size()
    }

    pub fn minimized(&self) -> bool {
        self.dimensions().width == 0 || self.dimensions().height == 0
    }

    pub fn begin_frame(&mut self) -> Result<()> {
        //
        // check if minized
        //
        if self.minimized() {
            return Ok(());
        }

        // It is important to call this function from time to time, otherwise resources
        // will keep accumulating and you will eventually reach an out of memory error.
        // Calling this function polls various fences in order to determine what the GPU
        // has already processed, and frees the resources that are no longer needed.
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        // get lock to the data
        let mut data = self.data.lock().unwrap();

        // Whenever the window resizes we need to recreate everything dependent on the
        // window size. In this example that includes the swapchain, the framebuffers and
        // the dynamic state viewport.
        if data.recreate_swapchain {
            // Use the new dimensions of the window.

            let (new_swapchain, new_swapchain_images) =
                match self
                    .swapchain
                    .recreate(vulkano::swapchain::SwapchainCreateInfo {
                        image_extent: self.dimensions().into(),
                        ..self.swapchain.create_info()
                    }) {
                    Ok(r) => r,
                    // This error tends to happen when the user is manually resizing the
                    // window. Simply restarting the loop is the easiest way to fix this
                    // issue.
                    Err(vulkano::swapchain::SwapchainCreationError::ImageExtentNotSupported {
                        ..
                    }) => return Ok(()),
                    Err(e) => panic!("failed to recreate swapchain: {e}"),
                };

            // save new swapchain
            self.swapchain = new_swapchain;
            self.swapchain_images = new_swapchain_images;

            // Because framebuffers contains a reference to the old swapchain, we need to
            // recreate framebuffers as well.
            (self.viewport, self.framebuffers) = window_size_dependent_setup(
                &self.swapchain_images,
                self.render_pass.clone(),
                &self.viewport,
            );

            // swapchain has been recreated.
            data.recreate_swapchain = false;
        }

        //
        // Before we can draw on the output, we have to *acquire* an image from the
        // swapchain. If no image is available (which happens if you submit draw commands
        // too quickly), then the function will block. This operation returns the index of
        // the image that we are allowed to draw upon.
        //
        // This function can block if no image is available. The parameter is an optional
        // timeout after which the function call will return an error.
        //
        let (image_index, suboptimal, acquire_future) =
            match vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(vulkano::swapchain::AcquireError::OutOfDate) => {
                    self.data.lock().unwrap().recreate_swapchain = true;
                    return Ok(());
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        // update
        self.suboptimal = suboptimal;
        self.acquire_future = Some(acquire_future);
        self.image_index = image_index;

        //
        // `acquire_next_image` can be successful, but suboptimal. This means that the
        // swapchain image will still work, but it may not display correctly. With some
        // drivers this can be when the window resizes, but it may not cause the swapchain
        // to become out of date.
        //
        if self.suboptimal {
            //
            // force recreation of swapchain
            //
            data.recreate_swapchain = true;
        }

        // all went fine
        Ok(())
    }

    pub fn _render(&self) -> Result<()> {
        // all went fine
        Ok(())
    }

    pub fn end_frame(
        &mut self,
        command_buffer: vulkano::command_buffer::PrimaryAutoCommandBuffer,
    ) -> Result<()> {
        // check if valid
        if !self.acquire_future.is_none() {
            //
            let future = self
                .previous_frame_end
                .take()
                .unwrap()
                .join(self.acquire_future.take().unwrap())
                .then_execute(self.queue.clone(), command_buffer)
                .unwrap()
                // The color output is now expected to contain our triangle. But in order to
                // show it on the screen, we have to *present* the image by calling
                // `then_swapchain_present`.
                //
                // This function does not actually present the image immediately. Instead it
                // submits a present command at the end of the queue. This means that it will
                // only be presented once the GPU has finished executing the command buffer
                // that draws the triangle.
                .then_swapchain_present(
                    self.queue.clone(),
                    vulkano::swapchain::SwapchainPresentInfo::swapchain_image_index(
                        self.swapchain.clone(),
                        self.image_index,
                    ),
                )
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    self.previous_frame_end = Some(future.boxed());
                }
                Err(vulkano::sync::FlushError::OutOfDate) => {
                    self.data.lock().unwrap().recreate_swapchain = true;
                    self.previous_frame_end = Some(vulkano::sync::now(self.device.clone()).boxed());
                }
                Err(e) => {
                    panic!("failed to flush future: {e}");
                    // previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
            }
        }

        // all went fine
        Ok(())
    }
}

fn window_size_dependent_setup(
    images: &[Arc<vulkano::image::SwapchainImage>],
    render_pass: Arc<vulkano::render_pass::RenderPass>,
    viewport: &vulkano::pipeline::graphics::viewport::Viewport,
) -> (
    vulkano::pipeline::graphics::viewport::Viewport,
    Vec<Arc<vulkano::render_pass::Framebuffer>>,
) {
    // get the dimensions
    let dimensions = images[0].dimensions().width_height();
    (
        // create new view port using source
        vulkano::pipeline::graphics::viewport::Viewport {
            origin: [viewport.origin[0], viewport.origin[1]],
            dimensions: [dimensions[0] as f32, dimensions[1] as f32],
            depth_range: viewport.depth_range.clone(),
        },
        // loop over images and map them to framebuffers
        images
            .iter()
            .map(|image| {
                let view = vulkano::image::view::ImageView::new_default(image.clone()).unwrap();
                vulkano::render_pass::Framebuffer::new(
                    render_pass.clone(),
                    vulkano::render_pass::FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>(),
    )
}

pub struct DeviceData {
    pub recreate_swapchain: bool,
}
