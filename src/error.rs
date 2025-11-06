#[non_exhaustive]
#[derive(thiserror::Error, Debug, Default)]
pub enum RendererError {
    #[default]
    #[error("Unknown error")]
    UnknownError,
    #[error("Unvalidated Vulkan Error: {0}")]
    VulkanError(#[from] vulkano::VulkanError),
    #[error("Validated Vulkan Error: {0}")]
    ValidatedVulkanError(#[from] vulkano::Validated<vulkano::VulkanError>),
    #[error("Vulkan Window Error: {0}")]
    VulkanWindowError(#[from] vulkano::swapchain::FromWindowError),
    #[error("Couldn't enumerate physical devices!")]
    EnumeratePhysicalDevice,
    #[error("Couldn't find a physical device!")]
    NoPhysicalDeviceFound,
    #[error("Couldn't find a graphical queue family!")]
    GraphicalQueueFamily,
    #[error("Failed to create a device and queues!")]
    FailedDeviceCreation,
}
