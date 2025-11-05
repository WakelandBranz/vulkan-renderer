pub(crate) mod batch;
pub(crate) mod device;
pub(crate) mod pipeline;
pub(crate) mod swapchain;
pub(crate) mod sync;
pub(crate) mod texture;
pub(crate) mod types;

use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    device::{Device, Queue, physical::PhysicalDevice},
    format::Format,
    image::Image,
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    render_pass::RenderPass,
    swapchain::{Surface, Swapchain},
};

pub(crate) fn create_instance(
    library: Arc<VulkanLibrary>,
) -> Result<Arc<Instance>, Box<dyn std::error::Error>> {
    Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    );

    todo!()
}

pub(crate) fn select_physical_device(
    instance: &Arc<Instance>,
) -> Result<Arc<PhysicalDevice>, Box<dyn std::error::Error>> {
    // Device selection logic
    todo!()
}

pub(crate) fn create_device_and_queue(
    physical_device: Arc<PhysicalDevice>,
) -> Result<(Arc<Device>, Arc<Queue>), Box<dyn std::error::Error>> {
    // Device + queue creation
    todo!()
}

pub(crate) fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
    window_size: [u32; 2],
) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>), Box<dyn std::error::Error>> {
    // Swapchain creation - use on resize
    todo!()
}

pub(crate) fn create_render_pass(
    device: Arc<Device>,
    format: Format,
) -> Result<Arc<RenderPass>, Box<dyn std::error::Error>> {
    // Render pass creation
    todo!()
}
