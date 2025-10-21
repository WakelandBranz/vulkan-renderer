mod device;
mod tests;

use std::sync::Arc;

use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::device::QueueFlags;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::{VulkanLibrary, instance};

pub struct VulkanRenderer {}

impl VulkanRenderer {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(VulkanRenderer {})
    }

    /*
    TODO: Make setup take in window information to actually set up Vulkan for the window.
    */
    pub fn setup(&self) -> Result<(), Box<dyn std::error::Error>> {
        // The InstanceCreateFlags::ENUMERATE_PORTABILITY flag is set to support devices, such as those on MacOS and iOS systems, that do not fully conform to the Vulkan Specification
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )?;

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

        // Please reference this https://vulkano.rs/03-buffer-creation/01-buffer-creation.html
        let iter = (0..128).map(|_| 5u8);
        self.create_uniform_buffer_from_iter(memory_allocator.clone(), iter)?;

        println!("Successfully created VulkanRenderer.");

        Ok(())
    }
}
