use std::sync::Arc;

use vulkano::{
    buffer::{AllocateBufferError, Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer}, memory::allocator::{AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter}, pipeline::graphics::vertex_input::Vertex, Validated, VulkanError
};

use crate::VulkanRenderer;

impl VulkanRenderer {
    /// Wrapper function for creating a buffer (for batching) from iter to send to GPU
    pub(crate) fn create_uniform_buffer_from_iter<T, I>(
        &self,
        memory_allocator: Arc<dyn MemoryAllocator>,
        iter: I,
    ) -> Result<Subbuffer<[T]>, Validated<AllocateBufferError>>
    where
        T: BufferContents,
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            iter,
        )
    }

    // For vertex data
    pub(crate) fn create_vertex_buffer<V: Vertex>(
        &self,
        vertices: &[V],
    ) -> Result<Arc<Buffer>, Box<dyn std::error::Error>> {
        todo!()
    }

    // For index data
    pub(crate) fn create_index_buffer(
        &self,
        indices: &[u32],
    ) -> Result<Arc<Buffer>, Box<dyn std::error::Error>> {
        todo!()
    }

    // For CPU-GPU transfers
    pub(crate) fn create_staging_buffer<T>(
        &self,
        data: &[T],
    ) -> Result<Arc<Buffer>, Box<dyn std::error::Error>> {
        todo!()
    }
}
