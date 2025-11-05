pub(crate) mod core;
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
        // The InstanceCreateFlags::ENUMERATE_PORTABILITY flag is set to support devices, such as those on MacOS and iOS systems, that do not fully conform to the Vulkan Specification
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )?;

        unsafe {
            let surface = Surface::from_window_ref(instance.clone(), window);
        }

        // Find a physical device which we can use to render (iGPU, GeForce/Radeon graphics cards, etc.)
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("Could not enumerate physical devices!")
            .next()
            .expect("No physical devices available!");

        // Gather the index of a viable queue family
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("Couldn't find a graphical queue family")
            as u32;

        println!(
            "Successfully chosen device {:?} running driver {:?} with version {:?}",
            physical_device.properties().device_name,
            physical_device.properties().driver_name.as_ref().unwrap(),
            physical_device.properties().driver_version
        );

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                // Here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("Failed to create a device!");

        // We now have an open channel of communication with a Vulkan device!
        // That being said, 'queues' is an iterator, but in this case it is just one device so we must extract it.
        let queue = queues.next().unwrap();

        // Remember, cloning device just clones the Arc which is inexpensive.
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
