use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, InstanceData, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::state::{RasteriserState, RendererState},
    resource_manager::resource_manager::{FramebufferID, ResourceIDTrait, ResourcesManager},
};

pub struct SkyStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    skybox: Option<Renderable>,
}

impl<'a> SkyStage<'a> {
    pub fn new(gl: &'a gl::Context, target: FramebufferID) -> Self {
        Self {
            gl,
            target,
            skybox: None,
        }
    }
}

impl<'a> PipelineStage for SkyStage<'a> {
    fn get_target(&self) -> FramebufferID {
        self.target
    }

    fn init(
        &mut self,
        memory_manager: &mut MemoryManager,
        resource_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
    }

    fn submit(&mut self, skybox: &Renderable) {
        self.skybox = Some(skybox.clone())
    }

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
        if let Some(skybox) = &self.skybox {
            renderer_state.set_rasteriser_state(RasteriserState {
                depth_func: gl::LEQUAL,
                cull_face: gl::FRONT,
                ..Default::default()
            });

            renderer_state.set_shader_program(skybox.shader_id, resources_manager);

            memory_manager.reserve_instance_space(1);

            let instance = InstanceData {
                material_index: skybox.material_id.index(),
                transform: skybox.transform,
            };

            let base_instance = memory_manager.get_instance_index();
            memory_manager.push_instance_data(&instance);

            upload_draw_data(memory_manager, resources_manager, skybox, 1, base_instance);
            make_draw_call(self.gl, memory_manager, 1);
        }
    }
}

fn upload_draw_data(
    memory_manager: &mut MemoryManager,
    resources_manager: &mut ResourcesManager,
    renderable: &Renderable,
    instance_count: u32,
    base_instance: u32,
) {
    let mesh = resources_manager.borrow_mesh(&renderable.mesh_id).unwrap();

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
            ((memory_manager.get_indirect_command_index() - command_count) * DRAW_COMMAND_SIZE)
                as i32,
            command_count as i32,
            0,
        );
    }
}
