use super::PipelineStage;
use crate::{
    components::Renderable,
    graphics::state::RasteriserState,
    memory_manager::memory_manager::MemoryManager,
    renderer::state::RendererState,
    resource_manager::resource_manager::{FramebufferID, ResourcesManager, ShaderProgramID},
};

pub struct AOStage {
    target: FramebufferID,
    shader_id: ShaderProgramID,
}

impl AOStage {
    pub fn new(target: FramebufferID, resources_manager: &mut ResourcesManager) -> Self {
        let shader_id = resources_manager.load_shader("res/shaders/ambient_occlusion.glsl");

        Self { target, shader_id }
    }
}

impl PipelineStage for AOStage {
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

    fn submit(&mut self, renderable_index: usize) {}

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
        rasteriser_state: &mut RasteriserState,
        renderables: &[Renderable],
    ) {
    }
}
