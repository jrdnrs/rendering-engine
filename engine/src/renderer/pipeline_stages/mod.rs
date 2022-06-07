use super::state::RendererState;
use crate::{
    components::Renderable,
    memory_manager::memory_manager::MemoryManager,
    resource_manager::resource_manager::{FramebufferID, ResourcesManager},
};

pub mod debug;
pub mod post_process;
pub mod scene;
pub mod shadow;
pub mod sky;

pub const STAGE_SCENE: u16 = 0b1000_0000_0000_0000;
pub const STAGE_SKY: u16 = 0b0100_0000_0000_0000;
pub const STAGE_POST_PROCESS: u16 = 0b0010_0000_0000_0000;
pub const STAGE_SHADOW: u16 = 0b0001_0000_0000_0000;

pub const STAGE_DEBUG: u16 = 0b0000_0000_0000_0001;

/// Pipeline executes stages in this order
pub const STAGES: &[u16] = &[STAGE_SHADOW, STAGE_SCENE, STAGE_SKY, STAGE_DEBUG, STAGE_POST_PROCESS];

pub trait PipelineStage {
    fn get_target(&self) -> FramebufferID;
    fn submit(&mut self, renderable: &Renderable);
    fn init(
        &mut self,
        memory_manager: &mut MemoryManager,
        resource_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    );
    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resource_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    );
}
