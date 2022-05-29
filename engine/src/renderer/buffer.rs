use glow::{self as gl, HasContext};
use log::{error, trace};

use super::asset::shader::{
    shader_data_element_count, shader_data_gl_type, shader_data_size_bytes, ShaderDataType,
};

pub fn gl_buffer_target(target: u32) -> &'static str {
    match target {
        0x8893 => "Element Array Buffer",
        0x8892 => "Vertex Array Buffer",
        0x8A11 => "Uniform Buffer",
        0x90D2 => "Shader Storage Buffer",
        0x8F3F => "Draw Indirect Buffer",
        _ => "Unknown",
    }
}

pub struct BufferElement {
    pub name: &'static str,
    pub type_: ShaderDataType,
    pub count: i32,
    pub offset: i32,
    pub normalised: bool,
}

impl BufferElement {
    pub fn new(type_: ShaderDataType, name: &'static str) -> Self {
        let count = shader_data_element_count(&type_);
        BufferElement {
            name,
            type_,
            count,
            offset: 0,
            normalised: false,
        }
    }
}
pub struct BufferLayout {
    pub elements: Vec<BufferElement>,
    pub stride: i32,
}

impl BufferLayout {
    pub fn new(mut elements: Vec<BufferElement>) -> Self {
        let mut offset = 0;
        for element in elements.iter_mut() {
            element.offset = offset;
            offset += shader_data_size_bytes(&element.type_);
        }

        BufferLayout {
            elements,
            stride: offset,
        }
    }
}

// pub struct VertexBuffer<'a> {
//     pub gl: &'a gl::Context,
//     pub handle: gl::Buffer,
//     pub buffer_index: u32,
//     pub layout: BufferLayout,
//     pub pointer: NonNull<u8>,
// }

// impl<'a> VertexBuffer<'a> {
//     pub fn new(gl: &'a gl::Context, layout: BufferLayout, size: i32) -> Self {
//         let vbo: gl::Buffer;

//         let flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;

//         let pointer = unsafe {
//             vbo = gl.create_buffer().unwrap();
//             gl.bind_buffer(gl::ARRAY_BUFFER, Some(vbo));
//             // gl.buffer_data_size(gl::ARRAY_BUFFER, size, gl::DYNAMIC_DRAW);
//             gl.buffer_storage(gl::ARRAY_BUFFER, size, None, flags);
//             let pointer =
//                 NonNull::new(gl.map_buffer_range(gl::ARRAY_BUFFER, 0, size, flags)).unwrap();
//             gl.bind_buffer(gl::ARRAY_BUFFER, None);

//             pointer
//         };

//         VertexBuffer {
//             gl,
//             handle: vbo,
//             buffer_index: 0,
//             layout,
//             pointer,
//         }
//     }

//     pub fn from_slice<T: bytemuck::Pod>(
//         gl: &'a gl::Context,
//         vertex_data: &[T],
//         layout: BufferLayout,
//     ) -> Self {
//         let data = bytemuck::cast_slice(vertex_data);
//         let vbo: gl::Buffer;

//         unsafe {
//             vbo = gl.create_buffer().unwrap();
//             gl.bind_buffer(gl::ARRAY_BUFFER, Some(vbo));
//             gl.buffer_data_u8_slice(gl::ARRAY_BUFFER, data, gl::STATIC_DRAW);
//             gl.bind_buffer(gl::ARRAY_BUFFER, None);
//         }

//         VertexBuffer {
//             gl,
//             handle: vbo,
//             buffer_index: data.len() as u32,
//             layout,
//             pointer: NonNull::new(0 as *mut _).unwrap(),
//         }
//     }

//     pub fn clear(&mut self) {
//         self.buffer_index = 0;
//     }

//     pub fn push_data<T: bytemuck::Pod>(&mut self, vertex_data: &[T]) {
//         let data: &[u8] = bytemuck::cast_slice(vertex_data);

//         unsafe {
//             // self.gl
//             //     .buffer_sub_data_u8_slice(gl::ARRAY_BUFFER, self.buffer_index as i32, data)
//             std::ptr::copy_nonoverlapping(
//                 data.as_ptr(),
//                 self.pointer.as_ptr().add(self.buffer_index as usize),
//                 data.len(),
//             );
//         }

//         self.buffer_index += data.len() as u32;
//     }

//     pub fn bind(&self) {
//         unsafe { self.gl.bind_buffer(gl::ARRAY_BUFFER, Some(self.handle)) }
//     }

//     pub fn unbind(&self) {
//         unsafe { self.gl.bind_buffer(gl::ARRAY_BUFFER, None) }
//     }
// }

// pub struct IndexBuffer<'a> {
//     pub gl: &'a gl::Context,
//     pub handle: gl::Buffer,
// }

// impl<'a> IndexBuffer<'a> {
//     pub fn from_slice<T: bytemuck::Pod>(gl: &'a gl::Context, indices: &[T]) -> Self {
//         let data = bytemuck::cast_slice(indices);
//         let ebo: gl::Buffer;

//         unsafe {
//             ebo = gl.create_buffer().unwrap();
//             gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(ebo));
//             gl.buffer_data_u8_slice(gl::ELEMENT_ARRAY_BUFFER, data, gl::STATIC_DRAW);
//             gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, None);
//         }

//         IndexBuffer {
//             handle: ebo,
//             gl,
//         }
//     }

//     pub fn push_data<T: bytemuck::Pod>(&mut self, indices: &[T]) {
//         let data = bytemuck::cast_slice(indices);

//         unsafe {
//             // self.gl.buffer_sub_data_u8_slice(
//             //     gl::ELEMENT_ARRAY_BUFFER,
//             //     self.buffer_index as i32,
//             //     data,
//             // )
//             std::ptr::copy_nonoverlapping(
//                 data.as_ptr(),
//                 self.pointer.as_ptr().add(self.buffer_index as usize),
//                 data.len(),
//             );
//         }

//         self.buffer_index += data.len() as u32;
//     }

//     /// updates indices by applying offset
//     pub fn update_and_push_data<T>(&mut self, indices: &[T], offset: T)
//     where
//         T: bytemuck::Pod + std::ops::AddAssign,
//     {
//         let mut indices_copy: Vec<T> = indices.to_owned();

//         for i in indices_copy.iter_mut() {
//             *i += offset;
//         }

//         let data: &[u8] = bytemuck::cast_slice(indices_copy.as_slice());

//         unsafe {
//             // self.gl.buffer_sub_data_u8_slice(
//             //     gl::ELEMENT_ARRAY_BUFFER,
//             //     self.buffer_index as i32,
//             //     data,
//             // )
//             std::ptr::copy_nonoverlapping(
//                 data.as_ptr(),
//                 self.pointer.as_ptr().add(self.buffer_index as usize),
//                 data.len(),
//             );
//         }

//         self.buffer_index += data.len() as u32;
//     }

//     pub fn bind(&self) {
//         unsafe {
//             self.gl
//                 .bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(self.handle))
//         }
//     }

//     pub fn unbind(&self) {
//         unsafe { self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, None) }
//     }
// }

pub struct VertexArray<'a> {
    pub gl: &'a gl::Context,
    pub handle: gl::VertexArray,
    pub layout: BufferLayout,
    pub vertex_buffer: BufferStorage<'a>,
    pub index_buffer: BufferStorage<'a>,
}

impl<'a> VertexArray<'a> {
    pub fn new(gl: &'a gl::Context, layout: BufferLayout, size: i32, multiple: i32) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };

        // TODO: These should definitely not all be the same size...
        let vbo = BufferStorage::new(gl, gl::ARRAY_BUFFER, size, multiple);
        let ebo = BufferStorage::new(gl, gl::ELEMENT_ARRAY_BUFFER, size, multiple);

        let vertex_array = VertexArray {
            gl,
            handle: vao,
            layout,
            vertex_buffer: vbo,
            index_buffer: ebo,
        };
        vertex_array.attach_vertex_buffers();
        vertex_array.attach_index_buffer();

        vertex_array
    }

    // pub fn element_count(&self) -> i32 {
    //     (self.index_buffer.buffer_index / 4) as i32
    // }

    // pub fn push_data<T: bytemuck::Pod>(&mut self, data: &[T], indices: &[T]) {
    //     let data_byte: &[u8] = bytemuck::cast_slice(data);
    //     let indices_byte: &[u8] = bytemuck::cast_slice(indices);

    //     self.vertex_buffer.push_data(data_byte);
    //     self.index_buffer.push_data(indices_byte);
    // }

    // /// updates indices by applying offset
    // pub fn update_index_push_data<T>(&mut self, data: &[T], indices: &[T], offset: T)
    // where
    //     T: bytemuck::Pod + std::ops::AddAssign,
    // {
    //     self.vertex_buffer.push_data(data);
    //     self.index_buffer.update_and_push_data(indices, offset);
    // }

    fn attach_vertex_buffers(&self) {
        self.bind();
        self.vertex_buffer.bind();

        for (i, element) in self.layout.elements.iter().enumerate() {
            unsafe {
                self.gl.vertex_attrib_pointer_f32(
                    i as u32,
                    element.count,
                    shader_data_gl_type(&element.type_),
                    false,
                    self.layout.stride,
                    element.offset,
                );
                self.gl.enable_vertex_attrib_array(i as u32);
            }
        }

        unsafe {
            const INDEX_LIMIT: usize = 1_000;

            let mut indices: [u16; INDEX_LIMIT] = [0; INDEX_LIMIT];
            for i in 0..INDEX_LIMIT {
                indices[i] = i as u16;
            }

            let data: &[u8] = bytemuck::cast_slice(&indices);

            let vbo = self.gl.create_buffer().unwrap();
            self.gl.bind_buffer(gl::ARRAY_BUFFER, Some(vbo));
            self.gl
                .buffer_data_u8_slice(gl::ARRAY_BUFFER, data, gl::STATIC_DRAW);

            let attr_index = self.layout.elements.len() as u32;
            self.gl
                .vertex_attrib_pointer_i32(attr_index, 1, gl::UNSIGNED_SHORT, 2, 0);
            self.gl.vertex_attrib_divisor(attr_index, 1);
            self.gl.enable_vertex_attrib_array(attr_index);
        }

        self.unbind();
        self.vertex_buffer.unbind();
    }

    fn attach_index_buffer(&self) {
        self.bind();
        self.index_buffer.bind();
        self.unbind();
        self.index_buffer.unbind();
    }

    pub fn bind(&self) {
        unsafe { self.gl.bind_vertex_array(Some(self.handle)) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.bind_vertex_array(None) }
    }
}

pub struct BufferLock {
    start: i32,
    length: i32,
    fence_handle: gl::Fence,
}

pub struct BufferLockManager<'a> {
    pub gl: &'a gl::Context,
    pub buffer_locks: Vec<BufferLock>,

    // temp for debug
    pub buffer_target: u32,
}

impl<'a> BufferLockManager<'a> {
    pub fn new(gl: &'a gl::Context, buffer_target: u32) -> Self {
        BufferLockManager {
            gl,
            buffer_locks: Vec::new(),
            buffer_target,
        }
    }

    fn check_range_intersect(&mut self, start: i32, length: i32) -> Option<BufferLock> {
        for (i, bl) in self.buffer_locks.iter().enumerate() {
            if start < (bl.start + bl.length) && bl.start < (start + length) {
                return Some(self.buffer_locks.remove(i));
            }
        }
        None
    }

    /// Checks if there is a current fence in progress for this range and, if so, blocks until it completes
    pub fn wait_for_locked_range(&mut self, start: i32, length: i32) {
        if let Some(bl) = self.check_range_intersect(start, length) {
            self.wait(bl)
        }
    }

    /// Adds a fence to GPU command stream
    pub fn lock_range(&mut self, start: i32, length: i32) {
        let fence_handle =
            unsafe { self.gl.fence_sync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0) }.unwrap();

        let bl = BufferLock {
            start,
            length,
            fence_handle,
        };

        self.buffer_locks.push(bl);
    }

    /// Waits for successful completion of fence conditions
    pub fn wait(&self, buffer_lock: BufferLock) {
        let mut wait_flags = 0;
        let mut wait_duration = 0;

        unsafe {
            loop {
                match self
                    .gl
                    .client_wait_sync(buffer_lock.fence_handle, wait_flags, wait_duration)
                {
                    gl::ALREADY_SIGNALED | gl::CONDITION_SATISFIED => {
                        // trace!("{}", format!(
                        //     "Completed for '{}' at range: {}-{}",
                        //     gl_buffer_target(self.buffer_target),
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
                                "Waiting for '{}' at range: {}-{}",
                                gl_buffer_target(self.buffer_target),
                                buffer_lock.start,
                                buffer_lock.start + buffer_lock.length
                            )
                        );
                        wait_flags = gl::SYNC_FLUSH_COMMANDS_BIT;
                        wait_duration = 1_000_000_000
                    }
                }
            }

            self.gl.delete_sync(buffer_lock.fence_handle);
        }
    }
}

pub struct BufferStorage<'a> {
    pub gl: &'a gl::Context,
    pub gl_target: u32,
    pub handle: gl::Buffer,
    pub buffer_lock_man: BufferLockManager<'a>,
    buffer_base_pointer: *mut u8,
    buffer_index: i32,
    section_size_bytes: i32,
    pub current_section: i32,
    sections: i32,
}

impl<'a> BufferStorage<'a> {
    pub fn new(gl: &'a gl::Context, gl_target: u32, size_bytes: i32, multiple: i32) -> Self {
        let handle: gl::Buffer;
        let buffer_base_pointer: *mut u8;
        let map_flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;
        let buffer_flags = map_flags | gl::DYNAMIC_STORAGE_BIT;

        unsafe {
            handle = gl.create_buffer().unwrap();
            gl.bind_buffer(gl_target, Some(handle));
            gl.buffer_storage(gl_target, size_bytes * multiple, None, buffer_flags);
            buffer_base_pointer =
                gl.map_buffer_range(gl_target, 0, size_bytes * multiple, map_flags);
            gl.bind_buffer(gl_target, None);
        }

        BufferStorage {
            gl,
            gl_target,
            handle,
            buffer_lock_man: BufferLockManager::new(gl, gl_target),
            buffer_base_pointer,
            buffer_index: 0,
            section_size_bytes: size_bytes,
            current_section: 0,
            sections: multiple,
        }
    }

    /// Round-robin to next section, if there are more than one
    pub fn next_section(&mut self) {
        self.current_section = (self.current_section + 1) % self.sections;
    }

    /// As buffer_index is a section-local index, this applies an offset with respect to current section
    pub fn current_section_buffer_index(&self) -> i32 {
        self.buffer_index + self.current_section_buffer_offset()
    }

    /// Offset with respect to current section
    pub fn current_section_buffer_offset(&self) -> i32 {
        self.section_size_bytes * self.current_section
    }

    /// Reset the section-local buffer index to 0
    pub fn reset_index(&mut self) {
        self.buffer_index = 0;
    }

    /// Wraps buffer_index if the required size_bytes can not fit contigiously in the buffer and waits for any 
    /// fences associated with the range to be met.
    pub fn reserve(&mut self, size_bytes: i32) {
        if self.buffer_index + size_bytes > self.section_size_bytes {
            self.buffer_index = 0;
        }

        self.buffer_lock_man
            .wait_for_locked_range(self.current_section_buffer_index(), size_bytes)
    }

    /// Sets a fence at the current point in the command stream. <br>
    /// The fence is associated with a range from _current_section_buffer_index_ to an offset determined by provided
    /// size_bytes.
    pub fn set_fence(&mut self, size_bytes: i32) {
        self.buffer_lock_man
            .lock_range(self.current_section_buffer_index(), size_bytes)
    }

    pub fn push_data_slice<T: bytemuck::Pod>(&mut self, data: &[T]) {
        let data: &[u8] = bytemuck::cast_slice(data);

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buffer_base_pointer
                    .add(self.current_section_buffer_index() as usize),
                data.len(),
            );
        }

        // it is assumed that user previously calls reserve on the range to avoid overflowing buffer
        self.buffer_index += data.len() as i32;
    }

    pub fn push_data<T>(&mut self, data: &T) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data as *const T as *const u8,
                self.buffer_base_pointer
                    .add(self.current_section_buffer_index() as usize),
                std::mem::size_of::<T>(),
            );
        }

        // it is assumed that user previously calls reserve on the range to avoid overflowing buffer
        self.buffer_index += std::mem::size_of::<T>() as i32;
    }

    pub fn set_data_slice<T: bytemuck::Pod>(&mut self, data: &[T], offset: usize) {
        let data: &[u8] = bytemuck::cast_slice(data);

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buffer_base_pointer
                    .add(self.current_section_buffer_offset() as usize + offset),
                data.len(),
            );
        }
    }

    pub fn set_data<T>(&mut self, data: &T, offset: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data as *const T as *const u8,
                self.buffer_base_pointer
                    .add(self.current_section_buffer_offset() as usize + offset),
                std::mem::size_of::<T>(),
            );
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.bind_buffer(self.gl_target, Some(self.handle)) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.bind_buffer(self.gl_target, None) }
    }
}
