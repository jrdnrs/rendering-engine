use super::PipelineStage;
use crate::{
    components::Renderable,
    graphics::{
        self,
        state::{Comparison, Orientation, RasteriserState},
    },
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, InstanceData, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::state::RendererState,
    resource_manager::resource_manager::{FramebufferID, ResourceIDTrait, ResourcesManager},
};

pub struct SkyStage {
    target: FramebufferID,
    skybox: Option<usize>,
}

impl SkyStage {
    pub fn new(target: FramebufferID) -> Self {
        Self {
            target,
            skybox: None,
        }
    }
}

impl PipelineStage for SkyStage {
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

    fn submit(&mut self, renderable_index: usize) {
        self.skybox = Some(renderable_index)
    }

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
        rasteriser_state: &mut RasteriserState,
        renderables: &[Renderable],
    ) {
        if let Some(skybox_index) = &self.skybox {
            let skybox = &renderables[*skybox_index];

            rasteriser_state.set(RasteriserState {
                depth_func: Comparison::EqualOrLess,
                cull_face: Orientation::Front,
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

            graphics::submit_draw_call(
                graphics::DrawMode::Triangles,
                graphics::DataType::Uint32,
                (memory_manager.get_indirect_command_index() - 1) * DRAW_COMMAND_SIZE,
                1,
            )
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
