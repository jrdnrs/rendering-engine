use std::mem::size_of;

use bytemuck::Pod;
use log::info;
use memoffset::offset_of;

use super::uniform_layouts::*;
use crate::{
    graphics::{buffer::*, shader::ShaderDataType},
    math::{Mat4f, Vec3f},
    resource_manager::model::VERTEX_SIZE,
};

/// The number of sections in each buffer
pub const BUFFERS: u32 = 3;

/// Not sure... <br>
/// Thought this would end up being vertices per frame, but seems it ends up acting as vertices per mesh
/// or MultiDrawIndirect call)?? Or it's neither - by virtue of the buffer being created as a vertex buffer, maybe
/// the writes are DMAd straight away and so I can just overwrite the buffer willynilly because the data has already
/// been moved?
const MAX_VERTICES: u32 = 50_000;

/// Commands per frame (unique meshes per shader per frame)
const MAX_COMMANDS: u32 = 1_000;

/// Instances per draw call
const MAX_INSTANCES: u32 = 100_000;

pub const DRAW_COMMAND_SIZE: u32 = size_of::<DrawElementsIndirectCommand>() as u32;
const INSTANCE_DATA_SIZE: u32 = size_of::<InstanceData>() as u32;

const VERTEX_BUFFER_SIZE: u32 = VERTEX_SIZE * MAX_VERTICES;
const INDIRECT_BUFFER_SIZE: u32 = DRAW_COMMAND_SIZE * MAX_COMMANDS;
const INSTANCE_BUFFER_SIZE: u32 = INSTANCE_DATA_SIZE * MAX_INSTANCES;
const STATIC_SHADER_STORAGE_BUFFER_SIZE: u32 = size_of::<StaticShaderStorageBuffers>() as u32;
const FRAME_SHADER_STORAGE_BUFFER_SIZE: u32 = size_of::<FrameShaderStorageBuffers>() as u32;
const DRAW_SHADER_STORAGE_BUFFER_SIZE: u32 = size_of::<DrawShaderStorageBuffers>() as u32;

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

pub struct MemoryManager {
    vertex_array: VertexArray,
    indirect_draw_buffer: BufferStorage,

    static_shader_storage_buffer: BufferStorage,
    frame_shader_storage_buffer: BufferStorage,
    draw_shader_storage_buffer: BufferStorage,

    buffer_lock: BufferLockManager,
}

impl MemoryManager {
    pub fn new() -> Self {
        let buffer_layouts = vec![
            BufferLayout::new(
                vec![
                    BufferElement::new(ShaderDataType::Float3, "positions"),
                    BufferElement::new(ShaderDataType::Float3, "normals"),
                    BufferElement::new(ShaderDataType::Float3, "tangents"),
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

        let vertex_array = VertexArray::new(buffer_layouts, MAX_VERTICES * 3 * 4, BUFFERS);

        let mm = Self {
            vertex_array,
            indirect_draw_buffer: BufferStorage::new(
                BufferType::DrawIndirectCommand,
                INDIRECT_BUFFER_SIZE,
                BUFFERS,
            ),

            static_shader_storage_buffer: BufferStorage::new(
                BufferType::Uniform,
                STATIC_SHADER_STORAGE_BUFFER_SIZE,
                1,
            ),
            frame_shader_storage_buffer: BufferStorage::new(
                BufferType::Uniform,
                FRAME_SHADER_STORAGE_BUFFER_SIZE,
                BUFFERS,
            ),
            draw_shader_storage_buffer: BufferStorage::new(
                BufferType::Uniform,
                DRAW_SHADER_STORAGE_BUFFER_SIZE,
                1,
            ),
            buffer_lock: BufferLockManager::new(),
        };

        mm.indirect_draw_buffer.bind();
        mm.vertex_array.bind();

        mm.bind_static_shader_storage_ranges();
        mm.bind_frame_shader_storage_ranges();
        mm.bind_draw_shader_storage_ranges();

        info!("Multi-buffering: {}", BUFFERS);
        info!(
            "Per Vertex Buffer Size: {:.3} MB",
            VERTEX_BUFFER_SIZE as f32 * BUFFERS as f32 / 1_000_000.0
        );
        info!(
            "Per Instance Buffer Size: {:.3} MB",
            INSTANCE_BUFFER_SIZE as f32 * BUFFERS as f32 / 1_000_000.0
        );
        info!(
            "Index Buffer Size: {:.3} MB",
            MAX_VERTICES as f32 * 3.0 * 4.0 * BUFFERS as f32 / 1_000_000.0
        );
        info!(
            "Indirect Draw Command Buffer Size: {:.3} MB",
            INDIRECT_BUFFER_SIZE as f32 * BUFFERS as f32 / 1_000_000.0
        );
        info!(
            "Static Shader Storage Buffer Size: {:.3} MB",
            STATIC_SHADER_STORAGE_BUFFER_SIZE as f32 * BUFFERS as f32 / 1_000_000.0
        );
        info!(
            "Per Frame Shader Storage Buffer Size: {:.3} MB",
            FRAME_SHADER_STORAGE_BUFFER_SIZE as f32 * BUFFERS as f32 / 1_000_000.0
        );
        info!(
            "Per Draw Call Shader Storage Buffer Size: {:.3} MB",
            DRAW_SHADER_STORAGE_BUFFER_SIZE as f32 * BUFFERS as f32 / 1_000_000.0
        );

        mm
    }

    // Section Management
    ///////////////////////////////////////////////////////////////////////////////////////

    fn bind_static_shader_storage_ranges(&self) {
        self.static_shader_storage_buffer.bind_buffer_range(
            0,
            offset_of!(StaticShaderStorageBuffers, materials) as u32,
            size_of::<MaterialsStorageBuffer>() as u32,
        );

        self.static_shader_storage_buffer.bind_buffer_range(
            1,
            offset_of!(StaticShaderStorageBuffers, shadow_maps) as u32,
            size_of::<ShadowMapStorageBuffer>() as u32,
        );
    }

    fn bind_frame_shader_storage_ranges(&self) {
        self.frame_shader_storage_buffer.bind_buffer_range(
            2,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + self.frame_shader_storage_buffer.buffer_section_offset,
            size_of::<LightsStorageBuffer>() as u32,
        );

        self.frame_shader_storage_buffer.bind_buffer_range(
            3,
            offset_of!(FrameShaderStorageBuffers, matrices) as u32
                + self.frame_shader_storage_buffer.buffer_section_offset,
            size_of::<MatricesStorageBuffer>() as u32,
        );
    }

    fn bind_draw_shader_storage_ranges(&self) {
        self.draw_shader_storage_buffer.bind_buffer_range(
            4,
            self.draw_shader_storage_buffer.current_buffer_index(),
            size_of::<GeneralPurposeStorageBuffer>() as u32,
        );
    }

    pub fn advance_sections(&mut self) {
        for buffer in self.vertex_array.vertex_buffers.iter_mut() {
            buffer.next_section();
            buffer.reset_index();
        }
        self.vertex_array.index_buffer.next_section();
        self.vertex_array.index_buffer.reset_index();

        self.indirect_draw_buffer.next_section();
        self.indirect_draw_buffer.reset_index();

        self.frame_shader_storage_buffer.next_section();
        self.frame_shader_storage_buffer.reset_index();
        self.bind_frame_shader_storage_ranges();
    }

    pub fn set_section_lock(&mut self) {
        self.buffer_lock
            .lock_range(self.indirect_draw_buffer.current_section, 1);
    }

    pub fn wait_for_section_lock(&mut self) {
        self.buffer_lock
            .wait_for_locked_range(self.indirect_draw_buffer.current_section, 1);
    }

    // Vertex Buffer 01 - per vertex
    ///////////////////////////////////////////////////////////////////////////////////////

    pub fn reserve_vertex_space(&mut self, vertex_count: u32) {
        self.vertex_array.vertex_buffers[0].reserve(vertex_count * VERTEX_SIZE);
    }

    pub fn get_vertex_index(&mut self) -> u32 {
        self.vertex_array.vertex_buffers[0].current_buffer_index() / VERTEX_SIZE
    }

    pub fn push_vertex_data<T>(&mut self, data: &T) {
        self.vertex_array.vertex_buffers[0].push_data(data)
    }

    pub fn push_vertex_slice<T: Pod>(&mut self, data: &[T]) {
        self.vertex_array.vertex_buffers[0].push_data_slice(data)
    }

    // Vertex Buffer 02 - per instance
    ///////////////////////////////////////////////////////////////////////////////////////

    pub fn reserve_instance_space(&mut self, vertex_count: u32) {
        self.vertex_array.vertex_buffers[1].reserve(vertex_count * INSTANCE_DATA_SIZE);
    }

    pub fn get_instance_index(&mut self) -> u32 {
        self.vertex_array.vertex_buffers[1].current_buffer_index() / INSTANCE_DATA_SIZE
    }

    pub fn push_instance_data<T>(&mut self, data: &T) {
        self.vertex_array.vertex_buffers[1].push_data(data)
    }

    pub fn push_instance_slice<T: Pod>(&mut self, data: &[T]) {
        self.vertex_array.vertex_buffers[1].push_data_slice(data)
    }

    // Index Buffer
    ///////////////////////////////////////////////////////////////////////////////////////

    pub fn reserve_index_space(&mut self, index_count: u32) {
        self.vertex_array.index_buffer.reserve(index_count * 4);
    }

    pub fn get_index_index(&mut self) -> u32 {
        self.vertex_array.index_buffer.current_buffer_index() / 4
    }

    pub fn push_index_data<T>(&mut self, data: &T) {
        self.vertex_array.index_buffer.push_data(data)
    }

    pub fn push_index_slice<T: Pod>(&mut self, data: &[T]) {
        self.vertex_array.index_buffer.push_data_slice(data)
    }

    // Indirect Draw Command Buffer
    ///////////////////////////////////////////////////////////////////////////////////////

    pub fn reserve_indirect_command_space(&mut self, command_count: u32) {
        self.indirect_draw_buffer
            .reserve(command_count * DRAW_COMMAND_SIZE);
    }

    pub fn get_indirect_command_index(&mut self) -> u32 {
        self.indirect_draw_buffer.current_buffer_index() / DRAW_COMMAND_SIZE
    }

    pub fn push_indirect_command(&mut self, command: DrawElementsIndirectCommand) {
        self.indirect_draw_buffer.push_data(&command)
    }

    // Static Shader Storage Buffer
    ///////////////////////////////////////////////////////////////////////////////////////

    pub fn set_material_data(&mut self, material: Material, index: u32) {
        self.static_shader_storage_buffer.set_data(
            &material,
            offset_of!(StaticShaderStorageBuffers, materials) as u32
                + offset_of!(MaterialsStorageBuffer, materials) as u32
                + (size_of::<Material>() as u32 * index),
        );
    }

    pub fn set_directional_shadow_map(&mut self, texture_handle: u64) {
        self.static_shader_storage_buffer.set_data(
            &texture_handle,
            offset_of!(StaticShaderStorageBuffers, shadow_maps) as u32
                + offset_of!(ShadowMapStorageBuffer, directional_shadow_map) as u32,
        );
    }

    pub fn set_spot_shadow_map(&mut self, texture_handle: u64, index: u32) {
        self.static_shader_storage_buffer.set_data(
            &texture_handle,
            offset_of!(StaticShaderStorageBuffers, shadow_maps) as u32
                + offset_of!(ShadowMapStorageBuffer, spot_shadow_map) as u32
                + (size_of::<ShadowMap>() as u32 * index),
        );
    }

    pub fn set_point_shadow_map(&mut self, texture_handle: u64, index: u32) {
        self.static_shader_storage_buffer.set_data(
            &texture_handle,
            offset_of!(StaticShaderStorageBuffers, shadow_maps) as u32
                + offset_of!(ShadowMapStorageBuffer, point_shadow_map) as u32
                + (size_of::<ShadowMap>() as u32 * index),
        );
    }

    // Per Frame Shader Storage Buffer
    ///////////////////////////////////////////////////////////////////////////////////////

    //// Point Lights
    ////////////////////////////

    pub fn set_point_light_data(&mut self, light: PointLight, index: u32) {
        self.frame_shader_storage_buffer.set_data(
            &light,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, point_lights) as u32
                + (size_of::<PointLight>() as u32 * index),
        );
    }

    pub fn set_point_light_data_slice(&mut self, light: &[PointLight]) {
        self.frame_shader_storage_buffer.set_data_slice(
            light,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, point_lights) as u32,
        );
    }

    pub fn set_point_light_count(&mut self, count: u32) {
        self.frame_shader_storage_buffer.set_data(
            &count,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, point_light_count) as u32,
        );
    }

    //// Spot Lights
    ////////////////////////////

    pub fn set_spot_light_data(&mut self, light: SpotLight, index: u32) {
        self.frame_shader_storage_buffer.set_data(
            &light,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, spot_lights) as u32
                + (size_of::<SpotLight>() as u32 * index),
        );
    }

    pub fn set_spot_light_data_slice(&mut self, light: &[SpotLight]) {
        self.frame_shader_storage_buffer.set_data_slice(
            light,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, spot_lights) as u32,
        );
    }

    pub fn set_spot_light_count(&mut self, count: u32) {
        self.frame_shader_storage_buffer.set_data(
            &count,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, spot_light_count) as u32,
        );
    }

    //// Directional Lights
    ////////////////////////////

    pub fn set_directional_light_data(&mut self, light: DirectionalLight) {
        self.frame_shader_storage_buffer.set_data(
            &light,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, directional_light) as u32,
        );
    }

    pub fn set_directional_light_count(&mut self, count: u32) {
        self.frame_shader_storage_buffer.set_data(
            &count,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, directional_light_count) as u32,
        );
    }

    //// Matrix Transforms
    ////////////////////////////

    pub fn set_projection_matrix(&mut self, projection_matrix: Mat4f) {
        self.frame_shader_storage_buffer.set_data(
            &projection_matrix,
            offset_of!(FrameShaderStorageBuffers, matrices) as u32
                + offset_of!(MatricesStorageBuffer, projection) as u32,
        );
    }

    pub fn set_view_matrix(&mut self, view_matrix: Mat4f) {
        self.frame_shader_storage_buffer.set_data(
            &view_matrix,
            offset_of!(FrameShaderStorageBuffers, matrices) as u32
                + offset_of!(MatricesStorageBuffer, view) as u32,
        );
    }

    //// Misc...
    ////////////////////////////

    pub fn set_camera_direction(&mut self, direction: Vec3f) {
        self.frame_shader_storage_buffer.set_data(
            &direction,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, camera_dir) as u32,
        );
    }

    pub fn set_camera_position(&mut self, position: Vec3f) {
        self.frame_shader_storage_buffer.set_data(
            &position,
            offset_of!(FrameShaderStorageBuffers, lights) as u32
                + offset_of!(LightsStorageBuffer, camera_pos) as u32,
        );
    }

    // Per Draw Call Shader Storage Buffer
    ///////////////////////////////////////////////////////////////////////////////////////

    pub fn reserve_per_draw_shader_data(&mut self, index_count: u32) {
        self.draw_shader_storage_buffer
            .reserve(index_count * size_of::<GeneralPurposeStorageBuffer>() as u32);
    }

    pub fn set_general(&mut self, data: GeneralPurposeStorageBuffer) {
        self.bind_draw_shader_storage_ranges();
        self.draw_shader_storage_buffer.push_data(&data);
    }

    pub fn set_general_index(&mut self, data: GeneralPurposeIndexStorageBuffer) {
        self.bind_draw_shader_storage_ranges();
        self.draw_shader_storage_buffer.set_data(
            &data,
            self.draw_shader_storage_buffer.current_buffer_index()
                + offset_of!(GeneralPurposeStorageBuffer, indices) as u32,
        );
        self.draw_shader_storage_buffer
            .increase_index(std::mem::size_of::<GeneralPurposeStorageBuffer>() as u32);
    }

    pub fn set_general_vec(&mut self, data: GeneralPurposeVecStorageBuffer) {
        self.bind_draw_shader_storage_ranges();
        self.draw_shader_storage_buffer.set_data(
            &data,
            self.draw_shader_storage_buffer.current_buffer_index()
                + offset_of!(GeneralPurposeStorageBuffer, vecs) as u32,
        );
        self.draw_shader_storage_buffer
            .increase_index(std::mem::size_of::<GeneralPurposeStorageBuffer>() as u32);
    }
}
