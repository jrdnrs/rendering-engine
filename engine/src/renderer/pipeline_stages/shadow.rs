use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::state::{RasteriserState, RendererState},
    resource_manager::{
        framebuffer::{Framebuffer, FramebufferAttachment, FramebufferConfig},
        resource_manager::{FramebufferID, MeshID, ResourcesManager, ShaderProgramID},
    },
};

const MAX_SHADOWS: usize = 4;

pub struct ShadowStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    shadow_maps: [FramebufferID; MAX_SHADOWS],
    shadow_shader_id: ShaderProgramID,
    renderables: Vec<Renderable>,
    total_instance_count: u32,
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
                internal_format: gl::DEPTH_COMPONENT,
            },
            width: 2048,
            height: 2048,
            ..Default::default()
        };

        let shadow_maps = [
            resources_manager.load_framebuffer(&config, false),
            resources_manager.load_framebuffer(&config, false),
            resources_manager.load_framebuffer(&config, false),
            resources_manager.load_framebuffer(&config, false),
        ];

        let shadow_shader_id = resources_manager.load_shader("res/shaders/shadow.glsl");

        Self {
            gl,
            target,
            shadow_maps,
            shadow_shader_id,
            renderables: Vec::new(),
            total_instance_count: 0,
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

    fn submit(&mut self, renderable: &Renderable) {}

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
        renderer_state.set_shader_program(self.shadow_shader_id, resources_manager);

        for shadow_map in self.shadow_maps.iter() {
            renderer_state.set_framebuffer(Some(shadow_map), resources_manager);
            unsafe {
                self.gl.viewport(0, 0, 2048, 2048);
                self.gl.clear(gl::DEPTH_BUFFER_BIT);
            }

            // upload matrix

            


        }


    }
}

fn upload_draw_data(
    memory_manager: &mut MemoryManager,
    resources_manager: &mut ResourcesManager,
    mesh_id: &MeshID,
    instance_count: u32,
    base_instance: u32,
) {
    let mesh = resources_manager.borrow_mesh(mesh_id).unwrap();

    memory_manager.reserve_vertex_space(mesh.vertices.len() as u32);
    memory_manager.reserve_index_space(mesh.indices.len() as u32);
    let indirect_command = DrawElementsIndirectCommand {
        count: mesh.indices.len() as u32,
        instance_count,
        first_index: memory_manager.get_index_index(),
        base_vertex: memory_manager.get_vertex_index(),
        base_instance,
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
            (memory_manager.get_indirect_command_index() 
                - command_count ) as u64 * DRAW_COMMAND_SIZE as u64,
            command_count as i32,
            0,
        );
    }
}
