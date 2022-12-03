#[cfg(feature = "opengl")]
pub use super::opengl::buffer::{BufferType, FenceHandle};
use super::{shader::ShaderDataType, ApiHandle};

// #[derive(Clone, Copy)]
// pub enum BufferType {
//     Vertex,
//     Index,
//     DrawIndirectCommand,
//     ShaderStorage,
//     Texture,
//     Uniform,
//     TransformFeedback,
// }

pub struct BufferElement {
    pub name: &'static str,
    pub data_type: ShaderDataType,
    pub count: usize,
    pub offset: usize,
    pub normalised: bool,
}

pub struct BufferLayout {
    pub elements: Vec<BufferElement>,
    pub stride: usize,
    pub buffer_size: usize,
    pub divisor: usize,
}

pub struct VertexArray {
    pub handle: ApiHandle,
    pub vertex_layouts: Vec<BufferLayout>,
    pub vertex_buffers: Vec<BufferStorage>,
    pub index_buffer: BufferStorage,
}

pub struct BufferStorage {
    pub handle: ApiHandle,
    pub buffer_type: BufferType,
    pub buffer_lock_man: BufferLockManager,
    pub buffer_base_pointer: *mut u8,

    pub buffer_section_offset: usize, // offset in entire buffer to the current section

    pub sections: usize,
    pub section_size_bytes: usize,
    pub current_section: usize,
    pub section_buffer_index: usize, // index into current section, not entire buffer
}

pub struct BufferLock {
    pub fence_handle: FenceHandle,
    pub start: usize,
    pub length: usize,
}

pub struct BufferLockManager {
    pub buffer_locks: Vec<BufferLock>,
}
