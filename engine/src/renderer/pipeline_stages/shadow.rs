use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    math::math::{Mat4f, Vec3f},
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, InstanceData, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::{
        camera::Camera,
        command::DrawCommands,
        state::{RasteriserState, RendererState},
    },
    resource_manager::{
        framebuffer::{Framebuffer, FramebufferAttachment, FramebufferConfig},
        resource_manager::{
            FramebufferID, MaterialID, MeshID, ResourceIDTrait, ResourcesManager, ShaderProgramID,
        },
    },
};

const MAX_SHADOWS: usize = 4;

pub struct ShadowStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    shadow_shader_id: ShaderProgramID,
    shadow_maps: [FramebufferID; MAX_SHADOWS],
    light_view: Camera,
    renderables: Vec<Renderable>,
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
        let config = FramebufferConfig {
            depth: FramebufferAttachment::Texture {
                internal_format: gl::DEPTH_COMPONENT32F,
            },
            width: 2048,
            height: 2048,
            ..Default::default()
        };

        let light_view = Camera::new_orthographic(1.0, 1.0, 20.0);
        memory_manager.set_light_projection(light_view.projection);

        let shadow_maps = [
            resources_manager.load_framebuffer(&config, false),
            resources_manager.load_framebuffer(&config, false),
            resources_manager.load_framebuffer(&config, false),
            resources_manager.load_framebuffer(&config, false),
        ];

        for (i, shadow_map_id) in shadow_maps.iter().enumerate() {
            let shadow_map = resources_manager.borrow_framebuffer(shadow_map_id).unwrap();

            let texture = shadow_map.get_depth_texture_handle().unwrap();
            let texture_handle = unsafe { gl.get_texture_handle(texture) };
            unsafe { gl.make_texture_handle_resident(texture_handle) }

            memory_manager.set_shadowmap_data(texture_handle.0.get(), i);
        }

        let shadow_shader_id = resources_manager.load_shader("res/shaders/shadow.glsl");

        Self {
            gl,
            target,
            shadow_shader_id,
            shadow_maps,
            light_view,
            renderables: Vec::new(),
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

    fn submit(&mut self, renderable: &Renderable) {
        self.renderables.push(renderable.clone())
    }

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
        renderer_state.set_rasteriser_state(RasteriserState {
            culling: false,
            ..Default::default()
        });

        renderer_state.set_shader_program(self.shadow_shader_id, resources_manager);

        self.command_queue.update_keys(&self.renderables);
        self.command_queue.sort_indices();

        // HACK: added a dummy renderable to the end to otherwise the last renderable would be cut off
        // when iterating through renderables with renderable and next_renderable. zip stops when one returns None
        self.renderables.push(Renderable {
            mesh_id: MeshID::new(0xFFFF),
            material_id: MaterialID::new(0xFFFF),
            shader_id: ShaderProgramID::new(0xFFFF),
            transform: Mat4f::identity(),
            pipeline_stages: 0,
        });
        self.command_queue.indices.push(self.renderables.len() - 1);

        let mut instance_count = 0;

        for (i, (index, next_index)) in self
            .command_queue
            .indices
            .iter()
            .zip(self.command_queue.indices[1..].iter())
            .enumerate()
        {
            let renderable = &self.renderables[*index];
            let next_renderable = &self.renderables[*next_index];

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
                    let renderable = &self.renderables[*instance_index];

                    memory_manager.push_instance_data(&InstanceData {
                        material_index: renderable.material_id.index(),
                        transform: renderable.transform.transpose(),
                    });
                }

                instance_count = 0;
            }
        }

        // for i in 0..renderer_state.lights.len().min(4) {
            self.light_view.direction = Vec3f::new(0.69, 0.23, -0.69).normalise();
            self.light_view.position = Vec3f::new(20.0, 4.0, -16.0);
            self.light_view.update_view();

            memory_manager.set_light_view_data(self.light_view.view.transpose(), 0);

            renderer_state.set_framebuffer(Some(&self.shadow_maps[0]), resources_manager);

            unsafe {
                self.gl.clear(gl::DEPTH_BUFFER_BIT);
            }

            make_draw_call(self.gl, memory_manager, self.pending_indirect_command_count);
        // }

        self.pending_indirect_command_count = 0;
        self.renderables.clear();
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
