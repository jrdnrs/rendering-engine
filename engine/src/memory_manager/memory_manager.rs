use std::mem::size_of;

use bytemuck::Pod;
use glow::{self as gl, HasContext};
use memoffset::offset_of;

use super::{buffer::*, uniform_layouts::*};
use crate::{
    math::math::{Mat4f, Vec2u, Vec3f},
    resource_manager::{model::VERTEX_SIZE, shader::ShaderDataType},
};

/// The number of sections
pub const BUFFERS: u32 = 2;

/// Not sure... <br>
/// Thought this would end up being vertices per frame, but seems it ends up acting as vertices per mesh
/// or MultiDrawIndirect call)?? Or it's neither - by virtue of the buffer being created as a vertex buffer, maybe
/// the writes are DMAd straight away and so I can just overwrite the buffer willynilly because the data has already
/// been moved?
const MAX_VERTICES: u32 = 500;

/// Commands per frame (unique meshes per shader per frame)
const MAX_COMMANDS: u32 = 1_000;

/// Instances per draw call
const MAX_INSTANCES: u32 = 100_000;

pub const DRAW_COMMAND_SIZE: u32 = size_of::<DrawElementsIndirectCommand>() as u32;
const INSTANCE_DATA_SIZE: u32 = size_of::<InstanceData>() as u32;

const VERTEX_BUFFER_SIZE: u32 = VERTEX_SIZE * MAX_VERTICES;
const INDIRECT_BUFFER_SIZE: u32 = DRAW_COMMAND_SIZE * MAX_COMMANDS;
const SHADER_STORAGE_BUFFER_SIZE: u32 = size_of::<ShaderStorageBuffers>() as u32;
const INSTANCE_BUFFER_SIZE: u32 = INSTANCE_DATA_SIZE * MAX_INSTANCES;

#[repr(C)]
pub struct DrawElementsIndirectCommand {
    pub count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub base_vertex: u32,
    pub base_instance: u32,
}

#[repr(C)]
pub struct InstanceData {
    pub material_index: u32,
    pub transform: Mat4f,
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
        let buffer_layouts = vec![
            BufferLayout::new(
                vec![
                    BufferElement::new(ShaderDataType::Float3, "positions"),
                    BufferElement::new(ShaderDataType::Float3, "normals"),
                    BufferElement::new(ShaderDataType::Float4, "colours"),
                    BufferElement::new(ShaderDataType::Float2, "tex_coords"),
                ],
                VERTEX_BUFFER_SIZE,
                0,
            ),
            BufferLayout::new(
                vec![
                    BufferElement::new(ShaderDataType::Uint1, "materialIndex"),
                    BufferElement::new(ShaderDataType::Float4, "transform_col1"),
                    BufferElement::new(ShaderDataType::Float4, "transform_col2"),
                    BufferElement::new(ShaderDataType::Float4, "transform_col3"),
                    BufferElement::new(ShaderDataType::Float4, "transform_col4"),
                ],
                INSTANCE_BUFFER_SIZE,
                1,
            ),
        ];

        let vertex_array = VertexArray::new(gl, buffer_layouts, 4_000, BUFFERS);

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
            buffer_lock: BufferLockManager::new(gl),
        };

        mm.indirect_draw_buffer.bind();
        mm.vertex_array.bind();

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
        }

        mm
    }

    pub fn advance_sections(&mut self) {
        for buffer in self.vertex_array.vertex_buffers.iter_mut() {
            buffer.next_section();
            buffer.reset_index();
        }
        self.vertex_array.index_buffer.next_section();
        self.indirect_draw_buffer.next_section();

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

    pub fn reserve_vertex_space(&mut self, vertex_count: u32) {
        self.vertex_array.vertex_buffers[0].reserve(vertex_count * VERTEX_SIZE);
    }

    pub fn reserve_instance_space(&mut self, vertex_count: u32) {
        self.vertex_array.vertex_buffers[1].reserve(vertex_count * INSTANCE_DATA_SIZE);
    }

    pub fn reserve_index_space(&mut self, index_count: u32) {
        self.vertex_array.index_buffer.reserve(index_count * 4);
    }

    // pub fn reset_vertex_index(&mut self) {
    //     self.vertex_array.vertex_buffers[0].reset_index();
    // }

    // pub fn reset_instance_index(&mut self) {
    //     self.vertex_array.vertex_buffers[1].reset_index();
    // }

    // pub fn reset_index_index(&mut self) {
    //     self.vertex_array.index_buffer.reset_index();
    // }

    pub fn get_vertex_index(&mut self) -> u32 {
        (self.vertex_array.vertex_buffers[0].current_buffer_index() / VERTEX_SIZE) as u32
    }

    pub fn get_instance_index(&mut self) -> u32 {
        (self.vertex_array.vertex_buffers[1].current_buffer_index() / INSTANCE_DATA_SIZE) as u32
    }

    pub fn get_index_index(&mut self) -> u32 {
        (self.vertex_array.index_buffer.current_buffer_index() / 4) as u32
    }

    pub fn get_indirect_command_index(&mut self) -> u32 {
        (self.indirect_draw_buffer.current_buffer_index() / DRAW_COMMAND_SIZE) as u32
    }

    pub fn push_vertex_data<T>(&mut self, data: &T) {
        self.vertex_array.vertex_buffers[0].push_data(data)
    }

    pub fn push_instance_data<T>(&mut self, data: &T) {
        self.vertex_array.vertex_buffers[1].push_data(data)
    }

    pub fn push_index_data<T>(&mut self, data: &T) {
        self.vertex_array.index_buffer.push_data(data)
    }

    pub fn push_vertex_slice<T: Pod>(&mut self, data: &[T]) {
        self.vertex_array.vertex_buffers[0].push_data_slice(data)
    }

    pub fn push_instance_slice<T: Pod>(&mut self, data: &[T]) {
        self.vertex_array.vertex_buffers[1].push_data_slice(data)
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
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, all_lights),
        );
    }

    pub fn set_light_view_data(&mut self, view: Mat4f, index: usize) {
        self.shader_storage_buffer.set_data(
            &view,
            offset_of!(ShaderStorageBuffers, lights)
                + offset_of!(LightsStorageBuffer, light_views)
                + size_of::<Mat4f>() * index,
        );
    }

    pub fn set_light_views_data(&mut self, views: &[Mat4f]) {
        self.shader_storage_buffer.set_data_slice(
            views,
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, light_views),
        );
    }

    pub fn set_light_count(&mut self, count: u32) {
        self.shader_storage_buffer.set_data(
            &count,
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, light_count),
        );
    }

    pub fn set_shadowmap_data(&mut self, handle: u64, index: usize) {
        self.shader_storage_buffer.set_data(
            &handle,
            offset_of!(ShaderStorageBuffers, lights)
                + offset_of!(LightsStorageBuffer, shadow_maps)
                + size_of::<u64>() * index,
        );
    }

    pub fn set_shadowmaps_data(&mut self, handles: &[u64]) {
        self.shader_storage_buffer.set_data_slice(
            handles,
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, shadow_maps),
        );
    }

    pub fn set_light_projection(&mut self, projection: Mat4f) {
        self.shader_storage_buffer.set_data(
            &projection,
            offset_of!(ShaderStorageBuffers, lights) + offset_of!(LightsStorageBuffer, light_projection),
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
}
