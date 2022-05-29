pub mod _cube;
pub mod asset;
pub mod buffer;
pub mod uniform_layouts;

use self::asset::{model::*, shader::*, *};
use crate::components::Renderable;
use crate::math::*;
use buffer::*;
use uniform_layouts::*;

use glow::{self as gl, HasContext};
use log::error;
use memoffset::offset_of;
use std::mem::size_of;

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

const DRAW_COMMAND_SIZE: i32 = size_of::<DrawElementsIndirectCommand>() as i32;

const BUFFER_SIZE: i32 = VERTEX_SIZE * MAX_VERTICES;
const INDIRECT_BUFFER_SIZE: i32 = DRAW_COMMAND_SIZE * MAX_COMMANDS;
const SHADER_STORAGE_BUFFER_SIZE: i32 =
    (size_of::<VertexUniform>() + size_of::<FragmentUniform>()) as i32;

#[repr(C)]
struct DrawElementsIndirectCommand {
    count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: u32,
    base_instance: u32,
}

pub struct Renderer<'a> {
    pub gl: &'a gl::Context,
    vertex_array: VertexArray<'a>,
    indirect_draw_buffer: BufferStorage<'a>,
    shader_storage_buffer: BufferStorage<'a>,
    pub asset_manager: AssetsManager<'a>,
    buffer_lock: BufferLockManager<'a>,
    queued_commands: usize,
    instance_index: usize,

    pub view_transform: Mat4f,
    pub projection_transform: Mat4f,

    pub camera_position: Vec3f,
    pub camera_direction: Vec3f,

    pub light_positions: Vec<Vec3f>,
}

impl<'a> Renderer<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        let layout = BufferLayout::new(vec![
            BufferElement::new(ShaderDataType::Float3, "positions"),
            BufferElement::new(ShaderDataType::Float3, "normals"),
            BufferElement::new(ShaderDataType::Float4, "colours"),
            BufferElement::new(ShaderDataType::Float2, "tex_coords"),
        ]);
        let vao = VertexArray::new(gl, layout, BUFFER_SIZE, BUFFERS);

        let r = Renderer {
            gl,
            vertex_array: vao,
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
            asset_manager: AssetsManager::new(gl),
            buffer_lock: BufferLockManager::new(gl, 0),
            queued_commands: 0,
            instance_index: 0,

            view_transform: Mat4f::identity(),
            projection_transform: Mat4f::identity(),
            camera_position: Vec3f::new(0.0, 0.0, 0.0),
            camera_direction: Vec3f::new(0.0, 0.0, 0.0),
            light_positions: Vec::new(),
        };
        r.init();

        r.indirect_draw_buffer.bind();
        r.shader_storage_buffer.bind();
        r.vertex_array.bind();
        r.vertex_array.vertex_buffer.bind();

        unsafe {
            r.gl.bind_buffer_range(
                gl::SHADER_STORAGE_BUFFER,
                0,
                Some(r.shader_storage_buffer.handle),
                0,
                size_of::<VertexUniform>() as i32,
            );

            r.gl.bind_buffer_range(
                gl::SHADER_STORAGE_BUFFER,
                1,
                Some(r.shader_storage_buffer.handle),
                offset_of!(Uniforms, fragment) as i32,
                size_of::<FragmentUniform>() as i32,
            );
        }
        r
    }

    fn init(&self) {
        unsafe {
            self.gl.enable(gl::DEPTH_TEST);

            self.gl.enable(gl::CULL_FACE);
            self.gl.cull_face(gl::BACK);
            self.gl.front_face(gl::CW);

            self.gl.enable(gl::BLEND);
            self.gl.blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            self.gl.debug_message_callback(|_, _, _, _, msg: &str| {
                error!("{}", msg);
            });
        }
    }

    pub fn set_viewport(&mut self, width: i32, height: i32) {
        self.projection_transform = Mat4f::perspective(
            width as f32 / height as f32,
            70.0f32.to_radians(),
            1.0,
            100.0,
        );

        unsafe {
            self.gl.viewport(0, 0, width, height);
        }
    }

    pub fn set_clear_colour(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.gl.clear_color(r, g, b, a) }
    }

    pub fn clear(&self) {
        unsafe { self.gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
    }

    pub fn begin(
        &mut self,
        view_transform: Mat4f,
        camera_position: Vec3f,
        camera_direction: Vec3f,
    ) {
        self.view_transform = view_transform;
        self.camera_position = camera_position;
        self.camera_direction = camera_direction;

        self.update_camera_data();

        self.buffer_lock
            .wait_for_locked_range(self.indirect_draw_buffer.current_section, 1);

        // wrapping the draw_indirect buffer is annoying but we then can't just send a list of contigious
        // commands, so just reset index
        // TODO: maybe this should be default behaviour when advancing sections?
        self.indirect_draw_buffer.reset_index();
        self.queued_commands = 0;
        self.instance_index = 0;
    }

    pub fn end(&mut self) {
        self.update_lights();
        self.batch();

        self.buffer_lock
            .lock_range(self.indirect_draw_buffer.current_section, 1);

        for shader_bucket in self.asset_manager.shader_buckets.iter_mut() {
            for mesh_bucket in shader_bucket.iter_mut() {
                mesh_bucket.clear()
            }
        }

        self.vertex_array.vertex_buffer.next_section();
        self.vertex_array.index_buffer.next_section();
        self.indirect_draw_buffer.next_section();
    }

    fn update_camera_data(&mut self) {
        let projection = self.projection_transform.transpose();
        let projection_offset =
            offset_of!(Uniforms, vertex) + offset_of!(VertexUniform, projection);
        self.shader_storage_buffer
            .set_data(&projection, projection_offset);

        let view = self.view_transform.transpose();
        let view_offset = offset_of!(Uniforms, vertex) + offset_of!(VertexUniform, view);
        self.shader_storage_buffer.set_data(&view, view_offset);

        let cam_dir_offset =
            offset_of!(Uniforms, fragment) + offset_of!(FragmentUniform, camera_dir);
        self.shader_storage_buffer
            .set_data(&self.camera_direction, cam_dir_offset);

        let cam_pos_offset =
            offset_of!(Uniforms, fragment) + offset_of!(FragmentUniform, camera_pos);
        self.shader_storage_buffer
            .set_data(&self.camera_position, cam_pos_offset);
    }

    fn update_lights(&mut self) {
        let all_lights_offset =
            offset_of!(Uniforms, fragment) + offset_of!(FragmentUniform, all_lights);

        for (i, light) in self.light_positions.iter().enumerate() {
            let l = LightUniform {
                ambient_strength: 0.1,
                diffuse_strength: 1.33,
                specular_strength: 0.5,
                inner_cutoff: 1.0,
                outer_cutoff: 1.0,
                quadratic: 0.0075,
                linear: 0.045,
                constant: 1.0,
                position: light.clone(),
                direction: Vec3f::new(0.0, 0.0, 0.0),
                ..Default::default()
            };

            self.shader_storage_buffer
                .set_data(&l, all_lights_offset + size_of::<LightUniform>() * i);
        }

        self.shader_storage_buffer.set_data(
            &(self.light_positions.len() as u32),
            offset_of!(Uniforms, fragment) + offset_of!(FragmentUniform, light_count),
        );
    }

    pub fn batch(&mut self) {
        for (shader_id, shader_bucket) in self.asset_manager.shader_buckets.iter().enumerate() {
            self.asset_manager.shader_program_manager.assets[shader_id].bind();

            for (mesh_id, mesh_bucket) in shader_bucket.iter().enumerate() {
                let mesh = &self.asset_manager.mesh_manager.assets[mesh_id];
                let vertex_count = mesh.vertices.len();
                let index_count = mesh.indices.len();

                self.vertex_array
                    .vertex_buffer
                    .reserve(vertex_count as i32 * VERTEX_SIZE);
                self.vertex_array
                    .index_buffer
                    .reserve((index_count * size_of::<u32>()) as i32);

                // push data to dib
                // we do this before pushing vbo and ebo data because we want those buffer indices to be prior to
                // data push so we have index to start of data
                self.indirect_draw_buffer
                    .push_data(&DrawElementsIndirectCommand {
                        count: mesh.indices.len() as u32,
                        instance_count: mesh_bucket.len() as u32,
                        first_index: self
                            .vertex_array
                            .index_buffer
                            .current_section_buffer_index()
                            as u32
                            / size_of::<u32>() as u32,
                        base_vertex: self
                            .vertex_array
                            .vertex_buffer
                            .current_section_buffer_index()
                            as u32
                            / VERTEX_SIZE as u32,
                        base_instance: self.instance_index as u32,
                    });
                self.queued_commands += 1;

                // push data to vbo and ebo
                self.vertex_array
                    .vertex_buffer
                    .push_data_slice(mesh.vertices.as_slice());

                self.vertex_array
                    .index_buffer
                    .push_data_slice(mesh.indices.as_slice());

                for renderable in mesh_bucket {
                    // push data to ssbo
                    self.shader_storage_buffer.set_data(
                        &(renderable.material_id.index()),
                        offset_of!(Uniforms, vertex)
                            + offset_of!(VertexUniform, material_index)
                            + size_of::<u32>() * self.instance_index,
                    );

                    self.shader_storage_buffer.set_data(
                        &renderable.transform.transpose(),
                        offset_of!(Uniforms, vertex)
                            + offset_of!(VertexUniform, transforms)
                            + size_of::<Mat4f>() * self.instance_index,
                    );

                    self.instance_index += 1;
                }

                // TODO: The buffer can only support MAX_COMMANDS number of commands per frame
                // so we should do something here, maybe other than glFinish
                // if self.instance_index as i32 >= MAX_INSTANCES {
                //     self.indirect_draw_buffer
                //         .push_data(&DrawElementsIndirectCommand {
                //             count: mesh.indices.len() as u32,
                //             instance_count: (i+1) as u32,
                //             first_index: indices_offset,
                //             base_vertex: vertices_offset,
                //             base_instance: self.instance_index as u32,
                //         });
                //     self.queued_commands += 1;

                //     gl_draw_batched_reset(
                //         self.gl,
                //         &mut self.indirect_draw_buffer,
                //         &mut self.instance_index,
                //         &mut self.queued_commands,
                //     );
                // }

                // }

                // If the current instance_index is less than the number of instances to render, we know that
                // we had to flush midway
                // let instance_count = self.instance_index.min(renderables.len()) as u32;

                // self.indirect_draw_buffer
                //     .push_data(&DrawElementsIndirectCommand {
                //         count: mesh.indices.len() as u32,
                //         instance_count,
                //         first_index: indices_offset,
                //         base_vertex: vertices_offset,
                //         base_instance: self.instance_index as u32,
                //     });
                // self.queued_commands += 1;
            }

            gl_draw_batched(
                self.gl,
                &mut self.indirect_draw_buffer,
                &mut self.queued_commands,
            );
        }
    }

    pub fn draw(&mut self, renderable: &Renderable) {
        let mesh_buckets =
            &mut self.asset_manager.shader_buckets[renderable.shader_id.index() as usize];

        if renderable.mesh_id.index() as usize >= mesh_buckets.len() {
            mesh_buckets.push(Vec::new())
        }

        mesh_buckets[renderable.mesh_id.index() as usize].push(renderable.clone());
    }

    pub fn load_shader(&mut self, path: &'static str) -> ShaderProgramID {
        self.asset_manager.load_shader(path)
    }

    pub fn load_mesh(&mut self, mesh: Mesh) -> MeshID {
        self.asset_manager.load_mesh(mesh)
    }

    pub fn load_material(&mut self, material: Material) -> MaterialID {
        let id = self.asset_manager.load_material(material);
        let index = id.index() as usize;
        let material = &self.asset_manager.material_manager.assets[index];

        let diff_texture_handle = unsafe {
            self.gl.get_texture_handle(
                self.asset_manager.texture_manager.assets
                    [material.diffuse_texture_id.index() as usize]
                    .handle,
            )
        };

        unsafe { self.gl.make_texture_handle_resident(diff_texture_handle) }

        let material_uniform = MaterialUniform {
            ambient_col: material.ambient_col,
            diffuse_col: material.diffuse_col,
            specular_col: material.specular_col,
            shininess: material.shininess,
            diffuse_texture: Vec2u::new(
                diff_texture_handle.0.get() as u32,
                (diff_texture_handle.0.get() >> 32) as u32,
            ),
            ..Default::default()
        };

        let material_offset = offset_of!(Uniforms, vertex)
            + offset_of!(VertexUniform, materials)
            + size_of::<MaterialUniform>() * index;
        self.shader_storage_buffer
            .set_data(&material_uniform, material_offset);

        id
    }

    pub fn load_texture(&mut self, path: &'static str) -> Result<TextureID, String> {
        self.asset_manager.load_texture(path)
    }
}

fn gl_draw_batched_reset(
    gl: &gl::Context,
    indirect_draw_buffer: &mut BufferStorage,
    instance_index: &mut usize,
    queued_commands: &mut usize,
) {
    gl_draw_batched(gl, indirect_draw_buffer, queued_commands);
    unsafe { gl.finish() }
    indirect_draw_buffer.reset_index();
    *instance_index = 0;
}

fn gl_draw_batched(
    gl: &gl::Context,
    indirect_draw_buffer: &mut BufferStorage,
    queued_commands: &mut usize,
) {
    // sends draw commands for this shader
    // TODO: this assumes one mesh per renderable, which doesn't have to always be the case
    // but I might change it so that a renderable just has a mesh instead of a model, not sure yet
    unsafe {
        gl.multi_draw_elements_indirect_offset(
            gl::TRIANGLES,
            gl::UNSIGNED_INT,
            indirect_draw_buffer.current_section_buffer_index()
                - DRAW_COMMAND_SIZE * *queued_commands as i32,
            *queued_commands as i32,
            0,
        );
    }
    *queued_commands = 0;
}
