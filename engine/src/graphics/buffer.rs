#[cfg(feature = "opengl")]
pub use super::opengl::buffer::{BufferType, FenceHandle};
use super::{shader::ShaderDataType, ApiHandle};


pub struct BufferElement {
    pub name: &'static str,
    pub data_type: ShaderDataType,
    pub count: u32,
    pub offset: u32,
    pub normalised: bool,
}

pub struct BufferLayout {
    pub elements: Vec<BufferElement>,
    pub stride: u32,
    pub buffer_size: u32,
    pub divisor: u32,
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

    pub buffer_section_offset: u32, // offset in entire buffer to the current section

    pub sections: u32,
    pub section_size_bytes: u32,
    pub current_section: u32,
    pub section_buffer_index: u32, // index into current section, not entire buffer
}

pub struct BufferLock {
    pub fence_handle: FenceHandle,
    pub start: u32,
    pub length: u32,
}

pub struct BufferLockManager {
    pub buffer_locks: Vec<BufferLock>,
}
