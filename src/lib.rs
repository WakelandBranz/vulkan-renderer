pub(crate) mod core;
pub mod error;
mod tests;

use std::sync::Arc;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use vulkano::{
    VulkanLibrary,
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    device::{Device, Queue},
    image::Image,
    instance::Instance,
    memory::allocator::StandardMemoryAllocator,
    render_pass::RenderPass,
    swapchain::{Surface, Swapchain},
};

use crate::{core::shaders, error::RendererError};

pub struct VulkanRenderer {
    // Core Vulkan objects
    pub(crate) instance: Arc<Instance>,
    pub(crate) surface: Arc<Surface>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    pub(crate) swapchain: Arc<Swapchain>,
    pub(crate) images: Vec<Arc<Image>>,
    pub(crate) render_pass: Arc<RenderPass>,

    // Memory
    pub(crate) memory_allocator: Arc<StandardMemoryAllocator>,
    pub(crate) command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

impl VulkanRenderer {
    /*
    TODO!:
    Make setup take in window information to actually set up Vulkan for the window.
    Turn all prints into logs using a logging crate
    */
    /// Creates a new Vulkan renderer.
    ///
    /// # Safety
    /// The window must outlive this renderer. Dropping the window before
    /// the renderer causes undefined behavior.
    pub fn new(
        window: &(impl HasWindowHandle + HasDisplayHandle),
        window_size: [u32; 2],
    ) -> Result<Self, RendererError> {
        let library = VulkanLibrary::new()?;
        let instance = core::create_instance(library, window)?;
        let surface = core::create_surface(instance.clone(), window)?;
        let physical_device = core::select_physical_device(instance.clone())?;
        let (device, queue) = core::create_device_and_queue(physical_device.clone())?;
        let (swapchain, images) =
            core::create_swapchain(device.clone(), surface.clone(), window_size)?;
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let render_pass = core::create_render_pass(device.clone(), swapchain.image_format())?;

        let shaders = shaders::Shaders::load(device.clone())?;
        

        // https://docs.rs/vulkano/0.34.0/vulkano/command_buffer/allocator/trait.CommandBufferAllocator.html
        // https://docs.rs/vulkano/0.34.0/vulkano/command_buffer/allocator/struct.StandardCommandBufferAllocator.html
        // TODO!: read more about secondary command buffers which can be found below
        // https://docs.rs/vulkano/0.34.0/vulkano/command_buffer/index.html
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));

        // Reminder, B8G8R8A8_SRGB with SrgbNonLinear is the standard image format choice for most games.

        println!("VulkanRenderer setup successful.");

        Ok(VulkanRenderer {
            // Core
            instance,
            device,
            queue,
            surface,
            swapchain,
            images,
            render_pass,

            // Memory
            memory_allocator,
            command_buffer_allocator,
        })
    }

    /// Call this when window is resized
    pub fn resize(&mut self, new_size: [u32; 2]) -> Result<(), RendererError> {
        let (swapchain, images) =
            core::create_swapchain(self.device.clone(), self.surface.clone(), new_size)?;

        self.swapchain = swapchain;
        self.images = images;

        Ok(())
    }
}
