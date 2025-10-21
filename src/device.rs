use std::sync::Arc;

use vulkano::{buffer::Buffer, pipeline::graphics::vertex_input::Vertex};

use crate::VulkanRenderer;

impl VulkanRenderer {
    // For vertex data
    pub fn create_vertex_buffer<V: Vertex>(
        &self,
        vertices: &[V],
    ) -> Result<Arc<Buffer>, Box<dyn std::error::Error>> {
        todo!()
    }

    // For index data
    pub fn create_index_buffer(
        &self,
        indices: &[u32],
    ) -> Result<Arc<Buffer>, Box<dyn std::error::Error>> {
        todo!()
    }

    // For CPU-GPU transfers
    pub fn create_staging_buffer<T>(
        &self,
        data: &[T],
    ) -> Result<Arc<Buffer>, Box<dyn std::error::Error>> {
        todo!()
    }
}
