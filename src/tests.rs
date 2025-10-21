#[cfg(test)]
mod tests {
    use crate::VulkanRenderer;

    use super::*;

    #[test]
    fn it_works() {
        let vulkan_renderer: VulkanRenderer = VulkanRenderer::init().expect("Failed init Vulkan Renderer in tests");
        vulkan_renderer.setup().expect("Failed to setup vulkan renderer.");
    }
}
