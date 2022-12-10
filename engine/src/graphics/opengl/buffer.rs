use log::{error, trace};

use super::shader::{shader_data_element_count, shader_data_gl_type, shader_data_size_bytes};
use crate::{
    graphics::{buffer::*, graphics::ApiEnum, shader::ShaderDataType},
    platform::rustgl as gl,
};

pub type FenceHandle = gl::GLsync;

#[derive(Clone, Copy)]
pub enum BufferType {
    Vertex = gl::ARRAY_BUFFER as isize,
    Index = gl::ELEMENT_ARRAY_BUFFER as isize,
    DrawIndirectCommand = gl::DRAW_INDIRECT_BUFFER as isize,
    ShaderStorage = gl::SHADER_STORAGE_BUFFER as isize,
    Texture = gl::TEXTURE_BUFFER as isize,
    Uniform = gl::UNIFORM_BUFFER as isize,
    TransformFeedback = gl::TRANSFORM_FEEDBACK_BUFFER as isize,
}

pub fn gl_buffer_target_to_string(target: ApiEnum) -> &'static str {
    match target {
        0x8893 => "Element Array Buffer",
        0x8892 => "Vertex Array Buffer",
        0x8A11 => "Uniform Buffer",
        0x90D2 => "Shader Storage Buffer",
        0x8F3F => "Draw Indirect Buffer",
        _ => "Unknown",
    }
}

impl BufferElement {
    pub fn new(data_type: ShaderDataType, name: &'static str) -> Self {
        let count = shader_data_element_count(&data_type);
        BufferElement {
            name,
            data_type,
            count,
            offset: 0,
            normalised: false,
        }
    }
}

impl BufferLayout {
    pub fn new(mut elements: Vec<BufferElement>, buffer_size: u32, divisor: u32) -> Self {
        let mut offset = 0;
        for element in elements.iter_mut() {
            element.offset = offset;
            offset += shader_data_size_bytes(&element.data_type);
        }

        BufferLayout {
            elements,
            stride: offset,
            buffer_size,
            divisor,
        }
    }
}

impl VertexArray {
    pub fn new(layouts: Vec<BufferLayout>, ebo_size: u32, multiple: u32) -> Self {
        let vao = unsafe { gl::create_named_vertex_array().unwrap() };

        let mut vbos = Vec::new();
        let ebo = BufferStorage::new(BufferType::Index, ebo_size, multiple);

        for layout in layouts.iter() {
            vbos.push(BufferStorage::new(
                BufferType::Vertex,
                layout.buffer_size,
                multiple,
            ))
        }

        let vertex_array = VertexArray {
            handle: vao.0,
            vertex_layouts: layouts,
            vertex_buffers: vbos,
            index_buffer: ebo,
        };
        vertex_array.attach_vertex_buffers();
        vertex_array.attach_index_buffer();

        vertex_array
    }

    pub fn set_divisor(&mut self, buffer_index: u32, divisor: u32) {
        if self.vertex_layouts[buffer_index as usize].divisor != divisor {
            self.vertex_layouts[buffer_index as usize].divisor = divisor;
            unsafe {
                gl::vertex_array_binding_divisor(
                    gl::GlVertexArray(self.handle),
                    buffer_index as u32,
                    divisor as u32,
                );
            }
        }
    }

    fn attach_vertex_buffers(&self) {
        let mut attr_index = 0;

        for (buffer_index, (layout, buffer)) in self
            .vertex_layouts
            .iter()
            .zip(self.vertex_buffers.iter())
            .enumerate()
        {
            unsafe {
                gl::vertex_array_vertex_buffer(
                    gl::GlVertexArray(self.handle),
                    buffer_index as u32,
                    Some(gl::GlBuffer(buffer.handle)),
                    0,
                    layout.stride as i32,
                )
            }

            for element in layout.elements.iter() {
                let gl_data_type = shader_data_gl_type(&element.data_type);

                unsafe {
                    if gl_data_type == gl::FLOAT {
                        gl::vertex_array_attrib_format_f32(
                            gl::GlVertexArray(self.handle),
                            attr_index as u32,
                            element.count as i32,
                            gl_data_type,
                            false,
                            element.offset as u32,
                        );
                    } else {
                        gl::vertex_array_attrib_format_i32(
                            gl::GlVertexArray(self.handle),
                            attr_index as u32,
                            element.count as i32,
                            gl_data_type,
                            element.offset as u32,
                        );
                    }

                    gl::vertex_array_attrib_binding_f32(
                        gl::GlVertexArray(self.handle),
                        attr_index as u32,
                        buffer_index as u32,
                    );
                    gl::enable_vertex_array_attrib(
                        gl::GlVertexArray(self.handle),
                        attr_index as u32,
                    );

                    if layout.divisor > 0 {
                        gl::vertex_array_binding_divisor(
                            gl::GlVertexArray(self.handle),
                            buffer_index as u32,
                            layout.divisor as u32,
                        );
                    }
                }

                attr_index += 1;
            }
        }
    }

    fn attach_index_buffer(&self) {
        unsafe {
            gl::vertex_array_element_buffer(
                gl::GlVertexArray(self.handle),
                Some(gl::GlBuffer(self.index_buffer.handle)),
            )
        }
    }

    pub fn bind(&self) {
        unsafe { gl::bind_vertex_array(Some(gl::GlVertexArray(self.handle))) }
    }

    pub fn unbind() {
        unsafe { gl::bind_vertex_array(None) }
    }
}

impl BufferLockManager {
    pub fn new() -> Self {
        BufferLockManager {
            buffer_locks: Vec::new(),
        }
    }

    fn check_range_intersect(&mut self, start: u32, length: u32) -> Option<BufferLock> {
        for (i, bl) in self.buffer_locks.iter().enumerate() {
            if start < (bl.start + bl.length) && bl.start < (start + length) {
                return Some(self.buffer_locks.remove(i));
            }
        }
        None
    }

    /// Checks if there is a current fence in progress for this range and, if so, blocks until it completes
    pub fn wait_for_locked_range(&mut self, start: u32, length: u32) {
        if let Some(bl) = self.check_range_intersect(start, length) {
            self.wait(bl)
        }
    }

    /// Adds a fence to GPU command stream
    pub fn lock_range(&mut self, start: u32, length: u32) {
        let fence_handle = unsafe { gl::fence_sync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0) }.unwrap();

        let bl = BufferLock {
            fence_handle: fence_handle.0,
            start,
            length,
        };

        self.buffer_locks.push(bl);
    }

    /// Waits for successful completion of fence conditions
    pub fn wait(&self, buffer_lock: BufferLock) {
        let mut wait_flags = 0;
        let mut wait_duration = 0;

        unsafe {
            loop {
                match gl::client_wait_sync(
                    gl::GlFence(buffer_lock.fence_handle),
                    wait_flags,
                    wait_duration,
                ) {
                    gl::ALREADY_SIGNALED | gl::CONDITION_SATISFIED => {
                        // trace!("{}", format!(
                        //     "Completed lock at range: {}-{}",
                        //     buffer_lock.start,
                        //     buffer_lock.start + buffer_lock.length
                        // ));
                        break;
                    }
                    gl::WAIT_FAILED => {
                        error!("Failed waiting for fence sync");
                        break;
                    }

                    _ => {
                        trace!(
                            "{}",
                            format!(
                                "Waiting for lock at range: {}-{}",
                                buffer_lock.start,
                                buffer_lock.start + buffer_lock.length
                            )
                        );
                        wait_flags = gl::SYNC_FLUSH_COMMANDS_BIT;
                        wait_duration = 1_000_000; // nanoseconds
                    }
                }
            }

            gl::delete_sync(gl::GlFence(buffer_lock.fence_handle));
        }
    }
}

impl BufferStorage {
    pub fn new(buffer_type: BufferType, size_bytes: u32, multiple: u32) -> Self {
        let handle: gl::GlBuffer;
        let map_flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;
        let buffer_flags = map_flags | gl::DYNAMIC_STORAGE_BIT;

        let buffer_base_pointer = unsafe {
            handle = gl::create_named_buffer().unwrap();
            gl::named_buffer_storage(handle, (size_bytes * multiple) as i32, None, buffer_flags);
            gl::map_named_buffer_range(handle, 0, (size_bytes * multiple) as i32, map_flags)
        };

        BufferStorage {
            handle: handle.0,
            buffer_type,
            buffer_lock_man: BufferLockManager::new(),
            buffer_base_pointer,

            buffer_section_offset: 0,

            sections: multiple,
            section_size_bytes: size_bytes,
            current_section: 0,
            section_buffer_index: 0,
        }
    }

    pub fn bind_buffer_range(&self, binding_index: u32, offset: u32, size: u32) {
        unsafe {
            gl::bind_buffer_range(
                self.buffer_type as u32,
                binding_index,
                Some(gl::GlBuffer(self.handle)),
                offset as i32,
                size as i32,
            )
        }
    }

    /// Round-robin to next section, if there are more than one
    pub fn next_section(&mut self) {
        self.current_section = (self.current_section + 1) % self.sections;
        self.buffer_section_offset = self.section_size_bytes * self.current_section;
    }

    /// As section_buffer_index is a section-local index, this applies an offset with respect to current section
    /// to get an offset into the entire buffer
    pub fn current_buffer_index(&self) -> u32 {
        self.section_buffer_index + self.buffer_section_offset
    }

    /// Reset the section-local buffer index to 0
    pub fn reset_index(&mut self) {
        self.section_buffer_index = 0;
    }

    /// Reset the section-local buffer index to 0
    pub fn increase_index(&mut self, increment: u32) {
        self.section_buffer_index += increment;
    }

    /// Wraps buffer_index if the required size_bytes can not fit contigiously in the buffer and waits for any
    /// fences associated with the range to be met.
    pub fn reserve(&mut self, size_bytes: u32) {
        if self.section_buffer_index + size_bytes > self.section_size_bytes {
            self.section_buffer_index = 0;
            // error!(
            //     "'{}' has overflowed, and mid frame sync is currently disabled",
            //     gl_buffer_target(self.gl_target)
            // )
        }

        self.buffer_lock_man
            .wait_for_locked_range(self.current_buffer_index(), size_bytes)
    }

    /// Sets a fence at the current point in the command stream. <br>
    /// The fence is associated with a range from _current_section_buffer_index_ to an offset determined by provided
    /// size_bytes.
    pub fn set_fence(&mut self, size_bytes: u32) {
        self.buffer_lock_man
            .lock_range(self.current_buffer_index(), size_bytes)
    }

    pub fn push_data_slice<T: bytemuck::Pod>(&mut self, data: &[T]) {
        let data: &[u8] = bytemuck::cast_slice(data);

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buffer_base_pointer
                    .add(self.current_buffer_index() as usize),
                data.len(),
            );
        }

        // it is assumed that user previously calls reserve on the range to avoid overflowing buffer
        self.section_buffer_index += data.len() as u32;
    }

    pub fn push_data<T>(&mut self, data: &T) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data as *const T as *const u8,
                self.buffer_base_pointer
                    .add(self.current_buffer_index() as usize),
                std::mem::size_of::<T>(),
            );
        }

        // it is assumed that user previously calls reserve on the range to avoid overflowing buffer
        self.section_buffer_index += std::mem::size_of::<T>() as u32;
    }

    pub fn set_data_slice<T: bytemuck::Pod>(&mut self, data: &[T], offset: u32) {
        let data: &[u8] = bytemuck::cast_slice(data);

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buffer_base_pointer
                    .add((self.buffer_section_offset + offset) as usize),
                data.len(),
            );
        }
    }

    pub fn set_data<T>(&mut self, data: &T, offset: u32) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data as *const T as *const u8,
                self.buffer_base_pointer
                    .add((self.buffer_section_offset + offset) as usize),
                std::mem::size_of::<T>(),
            );
        }
    }

    pub fn bind(&self) {
        unsafe { gl::bind_buffer(self.buffer_type as u32, Some(gl::GlBuffer(self.handle))) }
    }

    pub fn unbind(&self) {
        unsafe { gl::bind_buffer(self.buffer_type as u32, None) }
    }
}
