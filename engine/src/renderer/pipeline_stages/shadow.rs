use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    math::{Mat4f, Vec3f},
    memory_manager::{
        memory_manager::{
            DrawElementsIndirectCommand, InstanceData, MemoryManager, DRAW_COMMAND_SIZE,
        },
        uniform_layouts::{GeneralPurposeIndexStorageBuffer, MAX_POINT_LIGHTS, MAX_SPOT_LIGHTS},
    },
    renderer::{
        camera::Camera,
        command::DrawCommands,
        state::{RasteriserState, RendererState},
    },
    resource_manager::{
        framebuffer::{Framebuffer, FramebufferAttachment, FramebufferConfig, TextureType},
        resource_manager::{
            FramebufferID, MaterialID, MeshID, ResourceIDTrait, ResourcesManager, ShaderProgramID,
        },
    },
};

pub struct ShadowStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    dir_shadow_shader_id: ShaderProgramID,
    omni_shadow_shader_id: ShaderProgramID,

    point_shadows: [FramebufferID; MAX_POINT_LIGHTS],
    spot_shadows: [FramebufferID; MAX_SPOT_LIGHTS],
    directional_shadow: FramebufferID,

    renderable_indices: Vec<usize>,
    command_queue: DrawCommands,
    pending_indirect_command_count: u32,
}

impl<'a> ShadowStage<'a> {
    pub fn new(
        gl: &'a gl::Context,
        target: FramebufferID,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) -> Self {
        let dir_config = FramebufferConfig {
            depth: FramebufferAttachment::Texture {
                target: TextureType::T2D,
                internal_format: gl::DEPTH_COMPONENT32F,
                layers: 1,
                levels: 1
            },
            width: 4096,
            height: 4096,
            ..Default::default()
        };

        let omni_config = FramebufferConfig {
            depth: FramebufferAttachment::Texture {
                target: TextureType::T2DArray,
                internal_format: gl::DEPTH_COMPONENT32F,
                layers: 6,
                levels: 1
            },
            width: 512,
            height: 512,
            ..Default::default()
        };

        // set light projections
        renderer_state.light_ortho_projection = Mat4f::orthographic(80.0, 1.0, 80.0);
        renderer_state.light_persp_projection =
            Mat4f::perspective(1.0, 90f32.to_radians(), 1.0, 20.0);

        // init and upload point light shadow maps
        let mut point_shadows = [FramebufferID::new(0); MAX_POINT_LIGHTS];

        for i in 0..point_shadows.len() {
            point_shadows[i] = resources_manager.load_framebuffer(&omni_config, false);
        }

        for (i, shadow_map_id) in point_shadows.iter().enumerate() {
            let shadow_map = resources_manager.borrow_framebuffer(shadow_map_id).unwrap();

            let texture = shadow_map.get_depth_texture_handle().unwrap();
            let texture_handle = unsafe { gl.get_texture_handle(texture) };
            unsafe { gl.make_texture_handle_resident(texture_handle) }

            memory_manager.set_point_shadow_map(texture_handle.0.get(), i);
        }

        // init and upload spot light shadow maps
        let mut spot_shadows = [FramebufferID::new(0); MAX_SPOT_LIGHTS];

        for i in 0..spot_shadows.len() {
            spot_shadows[i] = resources_manager.load_framebuffer(&dir_config, false);
        }

        for (i, shadow_map_id) in spot_shadows.iter().enumerate() {
            let shadow_map = resources_manager.borrow_framebuffer(shadow_map_id).unwrap();

            let texture = shadow_map.get_depth_texture_handle().unwrap();
            let texture_handle = unsafe { gl.get_texture_handle(texture) };
            unsafe { gl.make_texture_handle_resident(texture_handle) }

            memory_manager.set_spot_shadow_map(texture_handle.0.get(), i);
        }

        // init and upload directional light shadow map
        let directional_shadow = resources_manager.load_framebuffer(&dir_config, false);
        let shadow_map = resources_manager
            .borrow_framebuffer(&directional_shadow)
            .unwrap();

        let texture = shadow_map.get_depth_texture_handle().unwrap();
        let texture_handle = unsafe { gl.get_texture_handle(texture) };
        unsafe { gl.make_texture_handle_resident(texture_handle) }
        memory_manager.set_directional_shadow_map(texture_handle.0.get());

        let dir_shadow_shader_id = resources_manager.load_shader("res/shaders/dir_shadow_map.glsl");
        let omni_shadow_shader_id =
            resources_manager.load_shader("res/shaders/cube_shadow_map.glsl");

        Self {
            gl,
            target,
            dir_shadow_shader_id,
            omni_shadow_shader_id,

            point_shadows,
            spot_shadows,
            directional_shadow,

            renderable_indices: Vec::new(),
            command_queue: DrawCommands::new(hash),
            pending_indirect_command_count: 0,
        }
    }
}

impl<'a> PipelineStage for ShadowStage<'a> {
    fn get_target(&self) -> FramebufferID {
        self.target
    }

    fn init(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
    }

    fn submit(&mut self, renderable_index: usize) {
        self.renderable_indices.push(renderable_index);
    }

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
        renderables: &[Renderable]
    ) {
        renderer_state.set_rasteriser_state(RasteriserState {
            cull_face: gl::FRONT,
            ..Default::default()
        });

        self.command_queue.update_keys(renderables, &self.renderable_indices);
        self.command_queue.sort_indices();

        // HACK: added a dummy renderable to the end to otherwise the last renderable would be cut off
        // when iterating through renderables with renderable and next_renderable. zip stops when one returns None
        // self.renderables.push(Renderable {
        //     mesh_id: MeshID::new(0xFFFF),
        //     material_id: MaterialID::new(0xFFFF),
        //     shader_id: ShaderProgramID::new(0xFFFF),
        //     transform: Mat4f::identity(),
        //     pipeline_stages: 0,
        // });
        // self.command_queue.indices.push(self.renderables.len() - 1);

        let mut instance_count = 0;

        for (i, (index, next_index)) in self
            .command_queue
            .indices
            .iter()
            .zip(self.command_queue.indices[1..].iter())
            .enumerate()
        {
            let renderable = &renderables[self.renderable_indices[*index]];
            let next_renderable = &renderables[self.renderable_indices[*next_index]];

            instance_count += 1;

            if renderable.mesh_id != next_renderable.mesh_id {
                memory_manager.reserve_instance_space(instance_count);
                upload_draw_data(
                    memory_manager,
                    resources_manager,
                    renderable,
                    instance_count,
                );
                self.pending_indirect_command_count += 1;

                for instance_index in
                    self.command_queue.indices[(i - (instance_count - 1) as usize)..(i + 1)].iter()
                {
                    let renderable = &renderables[self.renderable_indices[*instance_index]];

                    memory_manager.push_instance_data(&InstanceData {
                        material_index: renderable.material_id.index(),
                        transform: renderable.transform,
                    });
                }

                instance_count = 0;
            }
        }

        if renderer_state.point_lights.len() > 0 {
            renderer_state.set_shader_program(self.omni_shadow_shader_id, resources_manager);
        }

        for i in 0..MAX_POINT_LIGHTS.min(renderer_state.point_lights.len()) {
            renderer_state.set_framebuffer(Some(&self.point_shadows[i]), resources_manager);

            memory_manager.reserve_per_draw_shader_data(1);
            memory_manager.set_general_index(GeneralPurposeIndexStorageBuffer {
                index_1: i as u32,
                ..Default::default()
            });

            let fb = resources_manager
                .borrow_framebuffer(&self.point_shadows[i])
                .unwrap()
                .handle;
            unsafe {
                self.gl.clear_named_framebuffer_f32(fb, gl::DEPTH, 0, 1.0);
            }

            make_draw_call(self.gl, memory_manager, self.pending_indirect_command_count);
        }

        if renderer_state.spot_lights.len() > 0 || renderer_state.directional_light.is_some() {
            renderer_state.set_shader_program(self.dir_shadow_shader_id, resources_manager);
        }

        for i in 0..MAX_SPOT_LIGHTS.min(renderer_state.spot_lights.len()) {
            renderer_state.set_framebuffer(Some(&self.spot_shadows[i]), resources_manager);

            memory_manager.reserve_per_draw_shader_data(1);
            memory_manager.set_general_index(GeneralPurposeIndexStorageBuffer {
                index_1: 1 + i as u32,
                ..Default::default()
            });

            let fb = resources_manager
                .borrow_framebuffer(&self.spot_shadows[i])
                .unwrap()
                .handle;
            unsafe {
                self.gl.clear_named_framebuffer_f32(fb, gl::DEPTH, 0, 1.0);
            }

            make_draw_call(self.gl, memory_manager, self.pending_indirect_command_count);
        }

        if let Some(_directional_light) = renderer_state.directional_light {
            renderer_state.set_framebuffer(Some(&self.directional_shadow), resources_manager);

            memory_manager.reserve_per_draw_shader_data(1);
            memory_manager.set_general_index(GeneralPurposeIndexStorageBuffer {
                index_1: 0 as u32,
                ..Default::default()
            });

            let fb = resources_manager
                .borrow_framebuffer(&self.directional_shadow)
                .unwrap()
                .handle;
            unsafe {
                self.gl.clear_named_framebuffer_f32(fb, gl::DEPTH, 0, 1.0);
            }

            make_draw_call(self.gl, memory_manager, self.pending_indirect_command_count);
        }

        self.pending_indirect_command_count = 0;
        self.renderable_indices.clear();
    }
}

fn hash(r: &Renderable) -> u32 {
    r.mesh_id.index()
}

fn upload_draw_data(
    memory_manager: &mut MemoryManager,
    resources_manager: &mut ResourcesManager,
    renderable: &Renderable,
    instance_count: u32,
) {
    let mesh = resources_manager.borrow_mesh(&renderable.mesh_id).unwrap();

    memory_manager.reserve_vertex_space(mesh.vertices.len() as u32);
    memory_manager.reserve_index_space(mesh.indices.len() as u32);
    let indirect_command = DrawElementsIndirectCommand {
        count: mesh.indices.len() as u32,
        instance_count,
        first_index: memory_manager.get_index_index(),
        base_vertex: memory_manager.get_vertex_index(),
        base_instance: memory_manager.get_instance_index(),
    };
    memory_manager.push_indirect_command(indirect_command);
    memory_manager.push_vertex_slice(&mesh.vertices);
    memory_manager.push_index_slice(&mesh.indices);
}

fn make_draw_call(gl: &gl::Context, memory_manager: &mut MemoryManager, command_count: u32) {
    unsafe {
        gl.multi_draw_elements_indirect_offset(
            gl::TRIANGLES,
            gl::UNSIGNED_INT,
            ((memory_manager.get_indirect_command_index() - command_count) * DRAW_COMMAND_SIZE)
                as i32,
            command_count as i32,
            0,
        );
    }
}
