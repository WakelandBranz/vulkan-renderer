#[cfg(test)]
mod tests {
    use crate::VulkanRenderer;

    #[test]
    fn initialization_and_goofing_around() {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl3 demo", 800, 600)
            .position_centered()
            .vulkan()
            .build()
            .unwrap();

        // Setup
        let vulkan_renderer: VulkanRenderer =
            VulkanRenderer::new(window).expect("Failed init Vulkan Renderer in tests");

        // Part where we have fun

        // Test uniform buffer creation
        let iter = (0..128).map(|_| 5u8);
        vulkan_renderer
            .create_uniform_buffer_from_iter(iter)
            .expect("Failed to create uniform buffer from iter in test!");

        // Test source buffer creation
        let source_content: Vec<i32> = (0..64).collect();
        vulkan_renderer
            .create_source_buffer_from_iter(source_content)
            .expect("Failed to create uniform buffer from iter in test!");

        // Test destination buffer creation
        let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
        vulkan_renderer
            .create_destination_buffer_from_iter(destination_content)
            .expect("Failed to create uniform buffer from iter in test!");

        println!("ALL GOOD\nALL GOOD\nALL GOOD\n")
    }
}
