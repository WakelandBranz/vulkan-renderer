mod tests;

use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::{VulkanLibrary, instance};
use vulkano::device::QueueFlags;

pub struct VulkanRenderer {}

impl VulkanRenderer {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
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

        println!("Successfully chosen physical device: {:?}", );

        println!("Successfully created VulkanRenderer.");

        Ok(VulkanRenderer {})
    }
}
