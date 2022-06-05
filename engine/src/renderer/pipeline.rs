use std::collections::HashMap;

use glow::{self as gl, HasContext};

use super::{
    pipeline_stages::*,
    state::{RasteriserState, RendererState},
};
use crate::{
    components::Renderable, memory_manager::memory_manager::MemoryManager,
    resource_manager::resource_manager::{ResourcesManager, ResourceManagerTrait},
};

pub struct RendererPipeline<'a> {
    gl: &'a gl::Context,
    stages: HashMap<u16, Box<dyn PipelineStage + 'a>>,
}

impl<'a> RendererPipeline<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        Self {
            gl,
            stages: HashMap::new(),
        }
    }

    pub fn add_stage(&mut self, stage: impl PipelineStage + 'a, id: u16) {
        self.stages.insert(id, Box::new(stage));
    }

    pub fn submit(&mut self, renderable: &Renderable) {
        for (id, stage) in self.stages.iter_mut() {
            if *id & renderable.pipeline_stages == *id {
                stage.submit(renderable)
            }
        }
    }

    pub fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
        for stage_id in STAGES {
            if let Some(stage) = self.stages.get_mut(stage_id) {
                renderer_state.set_framebuffer(Some(stage.get_target()), resources_manager);
                stage.execute(memory_manager, resources_manager, renderer_state);
                renderer_state.set_rasteriser_state(RasteriserState::default());
            }
        }
    }
}
