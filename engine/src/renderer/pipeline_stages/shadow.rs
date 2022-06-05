use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::state::{RasteriserState, RendererState},
    resource_manager::resource_manager::{FramebufferID, MeshID, ResourcesManager},
};

pub struct ShadowStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
}

impl<'a> ShadowStage<'a> {
    pub fn new(gl: &'a gl::Context, target: FramebufferID) -> Self {
        Self { gl, target }
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

    memory_manager.reserve_vertex_space(mesh.vertices.len() as i32);
    memory_manager.reserve_index_space(mesh.indices.len() as i32);
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

fn make_draw_call(gl: &gl::Context, memory_manager: &mut MemoryManager, command_count: i32) {
    unsafe {
        gl.multi_draw_elements_indirect_offset(
            gl::TRIANGLES,
            gl::UNSIGNED_INT,
            (memory_manager.get_indirect_command_index() as i32 - command_count)
                * DRAW_COMMAND_SIZE,
            command_count,
            0,
        );
    }
}
