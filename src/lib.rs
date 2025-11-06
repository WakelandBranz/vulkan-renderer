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
    device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::StandardMemoryAllocator,
    swapchain::Surface,
};

pub struct VulkanRenderer {
    // Core Vulkan objects
    pub(crate) instance: Arc<Instance>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,

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
    pub fn new<W>(window: &W) -> Result<Self, Box<dyn std::error::Error>>
    where
        W: HasWindowHandle + HasDisplayHandle,
    {
        let library = VulkanLibrary::new()?;
        let instance = core::create_instance(library)?;
        let physical_device = core::select_physical_device(instance.clone())?;
        let (device, queue) = core::create_device_and_queue(physical_device.clone())?;

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        // https://docs.rs/vulkano/0.34.0/vulkano/command_buffer/allocator/trait.CommandBufferAllocator.html
        // https://docs.rs/vulkano/0.34.0/vulkano/command_buffer/allocator/struct.StandardCommandBufferAllocator.html
        // TODO!: read more about secondary command buffers which can be found below
        // https://docs.rs/vulkano/0.34.0/vulkano/command_buffer/index.html
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));

        /*
        This is how to use command buffers. Use it when you render a frame.
        Use CommandBufferUsage::OneTimeSubmit for dynamic frames, use CommandBufferUsage::MultipleSubmit for static things like UIs.
        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            queue_family_index,
            CommandBufferUsage::OneTimeSubmit,
        )?;

        let command_buffer = Arc::new(command_buffer_builder.build()?);
         */

        println!("VulkanRenderer setup successful.");

        Ok(VulkanRenderer {
            // Core
            instance,
            device,
            queue,

            // Memory
            memory_allocator,
            command_buffer_allocator,
        })
    }
}
