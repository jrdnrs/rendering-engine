use std::mem::size_of;

use glow::{self as gl, HasContext};
use log::{error, trace};

use crate::resource_manager::{
    model::VERTEX_SIZE,
    shader::{
        shader_data_element_count, shader_data_gl_type, shader_data_size_bytes, ShaderDataType,
    },
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
    pub buffer_size: u32,
    pub divisor: u32,
}

impl BufferLayout {
    pub fn new(mut elements: Vec<BufferElement>, buffer_size: u32, divisor: u32) -> Self {
        let mut offset = 0;
        for element in elements.iter_mut() {
            element.offset = offset;
            offset += shader_data_size_bytes(&element.type_);
        }

        BufferLayout {
            elements,
            stride: offset,
            buffer_size,
            divisor,
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
    pub vertex_layouts: Vec<BufferLayout>,
    pub vertex_buffers: Vec<BufferStorage<'a>>,
    pub index_buffer: BufferStorage<'a>,
}

impl<'a> VertexArray<'a> {
    pub fn new(
        gl: &'a gl::Context,
        layouts: Vec<BufferLayout>,
        ebo_size: u32,
        multiple: u32,
    ) -> Self {
        let vao = unsafe { gl.create_named_vertex_array().unwrap() };

        let mut vbos = Vec::new();
        let ebo = BufferStorage::new(gl, gl::ELEMENT_ARRAY_BUFFER, ebo_size, multiple);

        for layout in layouts.iter() {
            vbos.push(BufferStorage::new(
                gl,
                gl::ARRAY_BUFFER,
                layout.buffer_size,
                multiple,
            ))
        }

        let vertex_array = VertexArray {
            gl,
            handle: vao,
            vertex_layouts: layouts,
            vertex_buffers: vbos,
            index_buffer: ebo,
        };
        vertex_array.attach_vertex_buffers();
        vertex_array.attach_index_buffer();

        vertex_array
    }

    pub fn set_divisor(&mut self, buffer_index: usize, divisor: u32) {
        if self.vertex_layouts[buffer_index].divisor != divisor {
            self.vertex_layouts[buffer_index].divisor = divisor;
            unsafe {
                self.gl
                    .vertex_array_binding_divisor(self.handle, buffer_index as u32, divisor);
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
                self.gl.vertex_array_vertex_buffer(
                    self.handle,
                    buffer_index as u32,
                    Some(buffer.handle),
                    0,
                    layout.stride,
                )
            }

            for element in layout.elements.iter() {
                let gl_data_type = shader_data_gl_type(&element.type_);

                unsafe {
                    if gl_data_type == gl::FLOAT {
                        self.gl.vertex_array_attrib_format_f32(
                            self.handle,
                            attr_index as u32,
                            element.count,
                            gl_data_type,
                            false,
                            element.offset as u32,
                        );
                    } else {
                        self.gl.vertex_array_attrib_format_i32(
                            self.handle,
                            attr_index as u32,
                            element.count,
                            gl_data_type,
                            element.offset as u32,
                        );
                    }

                    self.gl.vertex_array_attrib_binding_f32(
                        self.handle,
                        attr_index as u32,
                        buffer_index as u32,
                    );
                    self.gl
                        .enable_vertex_array_attrib(self.handle, attr_index as u32);

                    if layout.divisor > 0 {
                        self.gl.vertex_array_binding_divisor(
                            self.handle,
                            buffer_index as u32,
                            layout.divisor,
                        );
                    }
                }

                attr_index += 1;
            }
        }
    }

    fn attach_index_buffer(&self) {
        unsafe {
            self.gl
                .vertex_array_element_buffer(self.handle, Some(self.index_buffer.handle))
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.bind_vertex_array(Some(self.handle)) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.bind_vertex_array(None) }
    }
}

pub struct BufferLock {
    start: u32,
    length: u32,
    fence_handle: gl::Fence,
}

pub struct BufferLockManager<'a> {
    pub gl: &'a gl::Context,
    pub buffer_locks: Vec<BufferLock>,
}

impl<'a> BufferLockManager<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        BufferLockManager {
            gl,
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

    pub buffer_section_offset: u32, // offset in entire buffer to the current section

    sections: u32,
    section_size_bytes: u32,
    pub current_section: u32,
    section_buffer_index: u32, // index into current section, not entire buffer
}

impl<'a> BufferStorage<'a> {
    pub fn new(gl: &'a gl::Context, gl_target: u32, size_bytes: u32, multiple: u32) -> Self {
        let handle: gl::Buffer;
        let map_flags = gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT;
        let buffer_flags = map_flags | gl::DYNAMIC_STORAGE_BIT;

        let buffer_base_pointer = unsafe {
            handle = gl.create_named_buffer().unwrap();
            gl.named_buffer_storage(handle, (size_bytes * multiple) as i32, None, buffer_flags);
            gl.map_named_buffer_range(handle, 0, (size_bytes * multiple) as i32, map_flags)
        };

        BufferStorage {
            gl,
            gl_target,
            handle,
            buffer_lock_man: BufferLockManager::new(gl),
            buffer_base_pointer,

            buffer_section_offset: 0,

            sections: multiple,
            section_size_bytes: size_bytes,
            current_section: 0,
            section_buffer_index: 0,
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

        // self.buffer_lock_man
        //     .wait_for_locked_range(self.current_buffer_index(), size_bytes)
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

    pub fn set_data_slice<T: bytemuck::Pod>(&mut self, data: &[T], offset: usize) {
        let data: &[u8] = bytemuck::cast_slice(data);

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buffer_base_pointer
                    .add(self.buffer_section_offset as usize + offset),
                data.len(),
            );
        }
    }

    pub fn set_data<T>(&mut self, data: &T, offset: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data as *const T as *const u8,
                self.buffer_base_pointer
                    .add(self.buffer_section_offset as usize + offset),
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
