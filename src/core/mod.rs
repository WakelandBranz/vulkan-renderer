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
    VulkanLibrary,
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
        physical::PhysicalDevice,
    },
    format::Format,
    image::{Image, ImageLayout, ImageUsage, SampleCount},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    render_pass::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
        RenderPass, RenderPassCreateInfo, SubpassDependency, SubpassDescription,
    },
    swapchain::{ColorSpace, Surface, Swapchain, SwapchainCreateInfo},
    sync::{AccessFlags, PipelineStages},
};

use crate::error::RendererError;

pub(crate) fn create_instance(
    library: Arc<VulkanLibrary>,
    window: &(impl HasWindowHandle + HasDisplayHandle),
) -> Result<Arc<Instance>, RendererError> {
    let required_extensions = Surface::required_extensions(window)?;

    Ok(Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
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
            enabled_extensions: DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::empty()
            },
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

// Swapchain creation - use on resize!!!!!
pub(crate) fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
    window_size: [u32; 2],
) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>), RendererError> {
    let physical_device = device.physical_device();

    // Query what the surface supports
    let surface_capabilities =
        physical_device.surface_capabilities(&surface, Default::default())?;

    // Select preferred format, if that fails get first available format.
    let (image_format, image_color_space) = physical_device
        .surface_formats(&surface, Default::default())?
        .into_iter()
        .find(|(format, color_space)| {
            *format == Format::B8G8R8A8_SRGB && *color_space == ColorSpace::SrgbNonLinear
        })
        .or_else(|| {
            physical_device
                .surface_formats(&surface, Default::default())
                .ok()?
                .into_iter()
                .next()
        })
        .ok_or(RendererError::NoSurfaceFormat)?;

    let (swapchain, images) = Swapchain::new(
        device,
        surface,
        SwapchainCreateInfo {
            min_image_count: surface_capabilities.min_image_count.max(2),

            image_format,
            image_color_space,
            image_extent: window_size,

            image_usage: ImageUsage::COLOR_ATTACHMENT,

            composite_alpha: surface_capabilities
                .supported_composite_alpha
                .into_iter()
                .next()
                .ok_or(RendererError::NoCompositeAlpha)?,

            ..Default::default()
        },
    )?;

    Ok((swapchain, images))
}

/// Creates a render pass for rendering directly to the swapchain.
pub(crate) fn create_render_pass(
    device: Arc<Device>,
    format: Format,
) -> Result<Arc<RenderPass>, RendererError> {
    // https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Render_passes
    let attachment = AttachmentDescription {
        format,
        samples: SampleCount::Sample1,
        load_op: AttachmentLoadOp::Clear, // Clear the values to a constant at the start
        store_op: AttachmentStoreOp::Store, // Rendered contents will be stored in memory and can be read later
        initial_layout: ImageLayout::Undefined,
        final_layout: ImageLayout::PresentSrc, // Ready for presentation
        ..Default::default()
    };

    let subpass = SubpassDescription {
        color_attachments: vec![Some(AttachmentReference {
            attachment: 0,
            layout: ImageLayout::ColorAttachmentOptimal,
            ..Default::default()
        })],
        ..Default::default()
    };

    let dependencies = vec![
        // Dependency from external to this subpass
        SubpassDependency {
            src_subpass: None, // External
            dst_subpass: Some(0),
            src_stages: PipelineStages::COLOR_ATTACHMENT_OUTPUT,
            dst_stages: PipelineStages::COLOR_ATTACHMENT_OUTPUT,
            src_access: AccessFlags::empty(),
            dst_access: AccessFlags::COLOR_ATTACHMENT_WRITE,
            ..Default::default()
        },
        // Dependency from this subpass to external (presentation)
        SubpassDependency {
            src_subpass: Some(0),
            dst_subpass: None, // External
            src_stages: PipelineStages::COLOR_ATTACHMENT_OUTPUT,
            dst_stages: PipelineStages::BOTTOM_OF_PIPE,
            src_access: AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_access: AccessFlags::empty(),
            ..Default::default()
        },
    ];

    Ok(RenderPass::new(
        device,
        RenderPassCreateInfo {
            attachments: vec![attachment],
            subpasses: vec![subpass],
            dependencies,
            ..Default::default()
        },
    )?)
}
