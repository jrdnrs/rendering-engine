use std::mem::size_of;

use bytemuck::Pod;
use glow::{self as gl, HasContext};
use memoffset::offset_of;

use super::{buffer::*, uniform_layouts::*};
use crate::{
    math::math::{Mat4f, Vec3f},
    resource_manager::{model::VERTEX_SIZE, shader::ShaderDataType},
};

/// The number of sections
pub const BUFFERS: i32 = 2;

/// Not sure... <br>
/// Thought this would end up being vertices per frame, but seems it ends up acting as vertices per mesh
/// or MultiDrawIndirect call)?? Or it's neither - by virtue of the buffer being created as a vertex buffer, maybe
/// the writes are DMAd straight away and so I can just overwrite the buffer willynilly because the data has already
/// been moved?
const MAX_VERTICES: i32 = 500;

/// Commands per frame
const MAX_COMMANDS: i32 = 1_000;

pub const DRAW_COMMAND_SIZE: i32 = size_of::<DrawElementsIndirectCommand>() as i32;

const BUFFER_SIZE: i32 = VERTEX_SIZE * MAX_VERTICES;
const INDIRECT_BUFFER_SIZE: i32 = DRAW_COMMAND_SIZE * MAX_COMMANDS;
const SHADER_STORAGE_BUFFER_SIZE: i32 = size_of::<ShaderStorageBuffers>() as i32;

#[repr(C)]
pub struct DrawElementsIndirectCommand {
    pub count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

pub struct MemoryManager<'a> {
    gl: &'a gl::Context,
    vertex_array: VertexArray<'a>,
    indirect_draw_buffer: BufferStorage<'a>,
    shader_storage_buffer: BufferStorage<'a>,

    buffer_lock: BufferLockManager<'a>,
}

impl<'a> MemoryManager<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        let buffer_layout = BufferLayout::new(vec![
            BufferElement::new(ShaderDataType::Float3, "positions"),
            BufferElement::new(ShaderDataType::Float3, "normals"),
            BufferElement::new(ShaderDataType::Float4, "colours"),
            BufferElement::new(ShaderDataType::Float2, "tex_coords"),
        ]);
        let vertex_array = VertexArray::new(gl, buffer_layout, BUFFER_SIZE, BUFFERS);

        let mm = Self {
            gl,
            vertex_array,
            indirect_draw_buffer: BufferStorage::new(
                gl,
                gl::DRAW_INDIRECT_BUFFER,
                INDIRECT_BUFFER_SIZE,
                BUFFERS,
            ),
            shader_storage_buffer: BufferStorage::new(
                gl,
                gl::SHADER_STORAGE_BUFFER,
                SHADER_STORAGE_BUFFER_SIZE,
                1,
            ),
            buffer_lock: BufferLockManager::new(gl, 0),
        };

        mm.indirect_draw_buffer.bind();
        mm.shader_storage_buffer.bind();
        mm.vertex_array.bind();
        mm.vertex_array.vertex_buffer.bind();

        unsafe {
            mm.gl.bind_buffer_range(
                gl::SHADER_STORAGE_BUFFER,
                0,
                Some(mm.shader_storage_buffer.handle),
                offset_of!(ShaderStorageBuffers, lights) as i32,
                size_of::<LightsStorageBuffer>() as i32,
            );

            mm.gl.bind_buffer_range(
                gl::SHADER_STORAGE_BUFFER,
                1,
                Some(mm.shader_storage_buffer.handle),
                offset_of!(ShaderStorageBuffers, skybox) as i32,
                size_of::<SkyboxStorageBuffer>() as i32,
            );

            mm.gl.bind_buffer_range(
                gl::SHADER_STORAGE_BUFFER,
                2,
                Some(mm.shader_storage_buffer.handle),
                offset_of!(ShaderStorageBuffers, materials) as i32,
                size_of::<MaterialsStorageBuffer>() as i32,
            );

            mm.gl.bind_buffer_range(
                gl::SHADER_STORAGE_BUFFER,
                3,
                Some(mm.shader_storage_buffer.handle),
                offset_of!(ShaderStorageBuffers, matrices) as i32,
                size_of::<MatricesStorageBuffer>() as i32,
            );

            mm.gl.bind_buffer_range(
                gl::SHADER_STORAGE_BUFFER,
                4,
                Some(mm.shader_storage_buffer.handle),
                offset_of!(ShaderStorageBuffers, instances) as i32,
                size_of::<InstanceStorageBuffer>() as i32,
            );
        }

        mm
    }

    pub fn advance_sections(&mut self) {
        self.vertex_array.vertex_buffer.next_section();
        self.vertex_array.index_buffer.next_section();
        self.indirect_draw_buffer.next_section();

        self.vertex_array.vertex_buffer.reset_index();
        self.vertex_array.index_buffer.reset_index();
        self.indirect_draw_buffer.reset_index();
    }

    pub fn set_section_lock(&mut self) {
        self.buffer_lock
            .lock_range(self.indirect_draw_buffer.current_section, 1);
    }

    pub fn wait_for_section_lock(&mut self) {
        self.buffer_lock
            .wait_for_locked_range(self.indirect_draw_buffer.current_section, 1);
    }

    pub fn reserve_vertex_space(&mut self, vertex_count: i32) {
        self.vertex_array
            .vertex_buffer
            .reserve(vertex_count * VERTEX_SIZE);
    }

    pub fn reserve_index_space(&mut self, index_count: i32) {
        self.vertex_array.index_buffer.reserve(index_count * 4);
    }

    pub fn get_vertex_index(&mut self) -> u32 {
        (self
            .vertex_array
            .vertex_buffer
            .current_section_buffer_index()
            / VERTEX_SIZE) as u32
    }

    pub fn get_index_index(&mut self) -> u32 {
        (self
            .vertex_array
            .index_buffer
            .current_section_buffer_index()
            / 4) as u32
    }

    pub fn get_indirect_command_index(&mut self) -> u32 {
        (self.indirect_draw_buffer.current_section_buffer_index() / DRAW_COMMAND_SIZE) as u32
    }

    pub fn push_vertex_data<T>(&mut self, data: &T) {
        self.vertex_array.vertex_buffer.push_data(data)
    }

    pub fn push_index_data<T>(&mut self, data: &T) {
        self.vertex_array.index_buffer.push_data(data)
    }

    pub fn push_vertex_slice<T: Pod>(&mut self, data: &[T]) {
        self.vertex_array.vertex_buffer.push_data_slice(data)
    }

    pub fn push_index_slice<T: Pod>(&mut self, data: &[T]) {
        self.vertex_array.index_buffer.push_data_slice(data)
    }

    pub fn push_indirect_command(&mut self, command: DrawElementsIndirectCommand) {
        self.indirect_draw_buffer.push_data(&command)
    }

    pub fn set_light_data(&mut self, light: Light, index: usize) {
        self.shader_storage_buffer.set_data(
            &light,
            offset_of!(ShaderStorageBuffers, lights)
                + offset_of!(LightsStorageBuffer, all_lights)
                + size_of::<Light>() * index,
        );
    }

    pub fn set_lights_data(&mut self, light: &[Light]) {
        self.shader_storage_buffer.set_data_slice(
            light,
            offset_of!(ShaderStorageBuffers, lights)
                + offset_of!(LightsStorageBuffer, all_lights)
        );
    }

    pub fn set_light_count(&mut self, count: u32) {
        self.shader_storage_buffer.set_data(
            &count,
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, light_count),
        );
    }

    pub fn set_camera_direction(&mut self, direction: Vec3f) {
        self.shader_storage_buffer.set_data(
            &direction,
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, camera_dir),
        );
    }

    pub fn set_camera_position(&mut self, position: Vec3f) {
        self.shader_storage_buffer.set_data(
            &position,
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, camera_pos),
        );
    }

    pub fn set_material_data(&mut self, material: Material, index: usize) {
        self.shader_storage_buffer.set_data(
            &material,
            offset_of!(ShaderStorageBuffers, materials)
                + offset_of!(MaterialsStorageBuffer, materials)
                + size_of::<Material>() * index,
        );
    }

    pub fn set_projection_matrix(&mut self, projection_matrix: Mat4f) {
        self.shader_storage_buffer.set_data(
            &projection_matrix,
            offset_of!(ShaderStorageBuffers, matrices)
                + offset_of!(MatricesStorageBuffer, projection),
        );
    }

    pub fn set_view_matrix(&mut self, view_matrix: Mat4f) {
        self.shader_storage_buffer.set_data(
            &view_matrix,
            offset_of!(ShaderStorageBuffers, matrices) + offset_of!(MatricesStorageBuffer, view),
        );
    }

    pub fn set_instance_data(&mut self, instance: InstanceData, index: usize) {
        self.shader_storage_buffer.set_data(
            &instance,
            offset_of!(ShaderStorageBuffers, instances)
                + offset_of!(InstanceStorageBuffer, instance_data)
                + size_of::<InstanceData>() * index,
        );
    }
}
