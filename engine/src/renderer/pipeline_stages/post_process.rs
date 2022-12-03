use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    memory_manager::memory_manager::MemoryManager,
    renderer::state::RendererState,
    resource_manager::resource_manager::{FramebufferID, ResourcesManager, ShaderProgramID},
};

pub struct PostProcessStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    shader_id: ShaderProgramID,
}

impl<'a> PostProcessStage<'a> {
    pub fn new(
        gl: &'a gl::Context,
        target: FramebufferID,
        resources_manager: &mut ResourcesManager<'a>,
    ) -> Self {
        let shader_id = resources_manager.load_shader("res/shaders/post_process_comp.glsl");

        Self {
            gl,
            target,
            shader_id,
        }
    }
}

impl<'a> PipelineStage for PostProcessStage<'a> {
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
        renderables: &[Renderable]
    ) {
        let fb = resources_manager.borrow_framebuffer(&self.target).unwrap();
        let texture = fb.get_colour_texture_handle().unwrap();

        let blocks_w = (fb.config.width + 15) / 16;
        let blocks_h = (fb.config.height + 15) / 16;

        renderer_state.set_shader_program(self.shader_id, resources_manager);

        unsafe {
            self.gl
                .bind_image_texture(0, texture, 0, false, 0, gl::READ_WRITE, gl::RGBA16F);

            self.gl.dispatch_compute(blocks_w + 1, blocks_h + 1, 1);
            self.gl.memory_barrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);

            self.gl.blit_to_default_framebuffer(
                fb.handle,
                0,
                0,
                fb.config.width as i32,
                fb.config.height as i32,
                0,
                0,
                fb.config.width as i32,
                fb.config.height as i32,
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            );
        }
    }
}
