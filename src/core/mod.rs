pub(crate) mod batch;
pub(crate) mod device;
pub(crate) mod pipeline;
pub(crate) mod swapchain;
pub(crate) mod sync;
pub(crate) mod texture;
pub(crate) mod types;

use std::sync::Arc;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use vulkano::{
    Validated, VulkanError, VulkanLibrary,
    device::{
        Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags, physical::PhysicalDevice,
    },
    format::Format,
    image::Image,
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    render_pass::RenderPass,
    swapchain::{Surface, Swapchain},
};

use crate::error::RendererError;

pub(crate) fn create_instance(library: Arc<VulkanLibrary>) -> Result<Arc<Instance>, RendererError> {
    Ok(Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )?)
}

/// Creates a Vulkan surface from a window.
///
/// # Safety Requirements
/// The window must remain alive for as long as the returned Surface exists.
/// Dropping the window before the surface causes undefined behavior.
pub(crate) fn create_surface(
    instance: Arc<Instance>,
    window: &(impl HasWindowHandle + HasDisplayHandle),
) -> Result<Arc<Surface>, RendererError> {
    Ok(unsafe { Surface::from_window_ref(instance.clone(), window)? })
}

pub(crate) fn select_physical_device(
    instance: Arc<Instance>,
) -> Result<Arc<PhysicalDevice>, RendererError> {
    // Find a physical device which we can use to render (iGPU, GeForce/Radeon graphics cards, etc.)
    instance
        .enumerate_physical_devices()?
        .next()
        .ok_or_else(|| RendererError::NoPhysicalDeviceFound)
}

// TODO: Let the user determine create info
pub(crate) fn create_device_and_queue(
    physical_device: Arc<PhysicalDevice>,
) -> Result<(Arc<Device>, Arc<Queue>), RendererError> {
    // Device + queue creation
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
        .ok_or_else(|| RendererError::GraphicalQueueFamily)? as u32;

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
    )?;

    /* TODO: TURN THIS INTO A DEBUG LOG!!!!!!!!!! 
    println!(
            "Successfully chosen device {:?} running driver {:?} with version {:?}",
            physical_device.properties().device_name,
            physical_device.properties().driver_name.as_ref().unwrap(),
            physical_device.properties().driver_version
        );
    */

    // We now have an open channel of communication with a Vulkan device!
    // That being said, 'queues' is an iterator, but in this case it is just one device so we must extract it.
    let queue = queues.next().unwrap();

    Ok((device, queue))
}

pub(crate) fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
    window_size: [u32; 2],
) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>), RendererError> {
    // Swapchain creation - use on resize
    todo!()
}

pub(crate) fn create_render_pass(
    device: Arc<Device>,
    format: Format,
) -> Result<Arc<RenderPass>, RendererError> {
    // Render pass creation
    todo!()
}
