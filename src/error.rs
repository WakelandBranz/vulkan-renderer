#[non_exhaustive]
#[derive(thiserror::Error, Debug, Default)]
pub enum RendererError {
    #[default]
    #[error("Unknown error")]
    UnknownError,

    // Vulkano/Vulkan Errors
    #[error("Unvalidated Vulkan Error: {0}")]
    VulkanError(#[from] vulkano::VulkanError),
    #[error("Validated Vulkan Error: {0}")]
    ValidatedVulkanError(#[from] vulkano::Validated<vulkano::VulkanError>),
    #[error("Library load error: {0}")]
    LibraryLoadFailure(#[from] vulkano::LoadingError),
    #[error("Vulkan Window Error: {0}")]
    VulkanWindowError(#[from] vulkano::swapchain::FromWindowError),
    #[error("No surface format available!")]
    NoSurfaceFormat,
    #[error("No composite alpha found for swapchain creation!")]
    NoCompositeAlpha,
    #[error("Couldn't enumerate physical devices!")]
    EnumeratePhysicalDevice,
    #[error("Couldn't find a physical device!")]
    NoPhysicalDeviceFound,
    #[error("Couldn't find a graphical queue family!")]
    GraphicalQueueFamily,
    #[error("Failed to create a device and queues!")]
    FailedDeviceCreation,

    // raw-window-handle errors
    #[error("raw-window-handle handle error: {0}")]
    HandleError(#[from] raw_window_handle::HandleError)
}
