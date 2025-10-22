use std::sync::Arc;

use vulkano::{
    Validated,
    buffer::{
        AllocateBufferError, Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer,
    },
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::graphics::vertex_input::Vertex,
};

use crate::VulkanRenderer;

impl VulkanRenderer {
    /*
    Wrapper functions for creating a buffer (for batching) from iter to send to GPU
    Currently building this out to cover all potential cases for future use. Not too hard.
     */
    pub(crate) fn create_uniform_buffer_from_iter<T, I>(
        &self,
        iter: I,
    ) -> Result<Subbuffer<[T]>, Validated<AllocateBufferError>>
    where
        T: BufferContents,
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Buffer::from_iter(
            self.memory_allocator.clone(),
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

    pub(crate) fn create_source_buffer_from_iter<T, I>(
        &self,
        iter: I,
    ) -> Result<Subbuffer<[T]>, Validated<AllocateBufferError>>
    where
        T: BufferContents,
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            iter,
        )
    }

    pub(crate) fn create_destination_buffer_from_iter<T, I>(
        &self,
        iter: I,
    ) -> Result<Subbuffer<[T]>, Validated<AllocateBufferError>>
    where
        T: BufferContents,
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
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
