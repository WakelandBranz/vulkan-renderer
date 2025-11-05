pub(crate) mod batch;
pub(crate) mod device;
pub(crate) mod pipeline;
pub(crate) mod swapchain;
pub(crate) mod sync;
pub(crate) mod texture;
pub(crate) mod types;

use std::sync::Arc;

use vulkano::VulkanLibrary;
use vulkano::device::{Device, Queue};
use vulkano::device::physical::PhysicalDevice;
use vulkano::image::Image;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::swapchain::{Surface, Swapchain};

pub(crate) fn create_instance(library: Arc<VulkanLibrary>) -> Result<Arc<Instance>, ...> {
    Instance::new(library, InstanceCreateInfo {
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        ..Default::default()
    })
}

pub(crate) fn select_physical_device(instance: &Arc<Instance>) -> Result<Arc<PhysicalDevice>, ...> {
    // Device selection logic
    todo!()
}

pub(crate) fn create_device_and_queue(
    physical_device: Arc<PhysicalDevice>
) -> Result<(Arc<Device>, Arc<Queue>), ...> {
    // Device + queue creation
    todo!()
}

pub(crate) fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
    window_size: [u32; 2],
) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>), ...> {
    // Swapchain creation - you'll reuse this on resize
}

pub(crate) fn create_render_pass(
    device: Arc<Device>,
    format: Format,
) -> Result<Arc<RenderPass>, ...> {
    // Render pass creation
}