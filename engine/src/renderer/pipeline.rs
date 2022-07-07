use std::collections::HashMap;

use glow::{self as gl, HasContext};
use nohash_hasher::BuildNoHashHasher;

use super::{
    pipeline_stages::*,
    state::{RasteriserState, RendererState},
};
use crate::{
    components::Renderable,
    memory_manager::memory_manager::MemoryManager,
    resource_manager::resource_manager::{ResourceManagerTrait, ResourcesManager},
};

pub struct RendererPipeline<'a> {
    gl: &'a gl::Context,
    stages: HashMap<u16, Box<dyn PipelineStage + 'a>, BuildNoHashHasher<u16>>,
    enabled: u16,
}

impl<'a> RendererPipeline<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        Self {
            gl,
            stages: HashMap::with_hasher(BuildNoHashHasher::default()),
            enabled: 0,
        }
    }

    pub fn add_stage(&mut self, stage: impl PipelineStage + 'a, id: u16) {
        self.stages.insert(id, Box::new(stage));
        self.enable_stages(id);
    }

    pub fn remove_stage(&mut self, id: u16) {
        self.disable_stages(id);
        self.stages.remove(&id);
    }

    pub fn enable_stages(&mut self, mask: u16) {
        self.enabled |= mask;
    }

    pub fn disable_stages(&mut self, mask: u16) {
        self.enabled &= !mask;
    }

    pub fn is_enabled(&mut self, mask: u16) -> bool {
        self.enabled & mask == mask
    }

    pub fn submit(&mut self, renderable: &Renderable) {
        for (id, stage) in self.stages.iter_mut() {
            if *id & self.enabled > 0 && *id & renderable.pipeline_stages == *id {
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
            if stage_id & self.enabled > 0 {
                if let Some(stage) = self.stages.get_mut(stage_id) {
                    renderer_state.set_framebuffer(Some(&stage.get_target()), resources_manager);
                    stage.execute(memory_manager, resources_manager, renderer_state);
                    renderer_state.set_rasteriser_state(RasteriserState::default());
                }
            }
        }

        renderer_state.set_framebuffer(None, resources_manager);
    }
}
