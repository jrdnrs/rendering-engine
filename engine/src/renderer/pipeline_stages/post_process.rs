use super::PipelineStage;
use crate::{
    components::Renderable,
    graphics::{
        self,
        framebuffer::{Framebuffer, FramebufferAttachment, InternalFormat},
        shader::Program,
        state::RasteriserState,
        texture::TextureFilter,
        AccessModifier, Barriers,
    },
    memory_manager::memory_manager::MemoryManager,
    renderer::state::RendererState,
    resource_manager::resource_manager::{FramebufferID, ResourcesManager, ShaderProgramID},
};

pub struct PostProcessStage {
    target: FramebufferID,
    shader_id: ShaderProgramID,
}

impl PostProcessStage {
    pub fn new(target: FramebufferID, resources_manager: &mut ResourcesManager) -> Self {
        let shader_id = resources_manager.load_shader("res/shaders/post_process_comp.glsl");

        Self { target, shader_id }
    }
}

impl PipelineStage for PostProcessStage {
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
        let fb = resources_manager.borrow_framebuffer(&self.target).unwrap();

        let blocks_w = (fb.config.width + 15) / 16;
        let blocks_h = (fb.config.height + 15) / 16;

        renderer_state.set_shader_program(self.shader_id, resources_manager);

        if let FramebufferAttachment::Texture(texture) = &fb.colour_handle {
            texture.bind_image_unit(0, 0, 0, AccessModifier::ReadWrite, InternalFormat::RGBA16F)
        }

        Program::dispatch_compute(blocks_w + 1, blocks_h + 1, 1);
        graphics::memory_barrier(Barriers::ShaderImageAccess as u32);
        fb.blit_to_default_framebuffer(
            0,
            0,
            fb.config.width,
            fb.config.height,
            0,
            0,
            fb.config.width,
            fb.config.height,
            TextureFilter::Nearest,
        );
    }
}
