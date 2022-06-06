use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    math::math::Vec4f,
    memory_manager::memory_manager::{
        DrawElementsIndirectCommand, MemoryManager, DRAW_COMMAND_SIZE,
    },
    renderer::state::{RasteriserState, RendererState},
    resource_manager::{
        framebuffer::{Framebuffer, FramebufferAttachment, FramebufferConfig},
        prefabs::quad_mesh,
        resource_manager::{FramebufferID, MeshID, ResourcesManager, ShaderProgramID},
    },
};

pub struct PostProcessStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    blit_buffer: FramebufferID,
    shader_id: ShaderProgramID,
    mesh_id: MeshID,
}

impl<'a> PostProcessStage<'a> {
    pub fn new(
        gl: &'a gl::Context,
        target: FramebufferID,
        resources_manager: &mut ResourcesManager<'a>,
    ) -> Self {
        let shader_id = resources_manager.load_shader("res/shaders/post_process.glsl");
        let mesh_id = resources_manager.load_mesh(quad_mesh(Vec4f::new(1.0, 1.0, 1.0, 1.0)));

        let config = FramebufferConfig {
            colour: FramebufferAttachment::Texture {
                internal_format: gl::RGBA16F,
            },
            depth: FramebufferAttachment::None,
            stencil: FramebufferAttachment::None,
            width: crate::WIDTH,
            height: crate::HEIGHT,
            samples: 1,
        };
        let blit_buffer = resources_manager.load_framebuffer(&config, true);

        Self {
            gl,
            target,
            blit_buffer,
            shader_id,
            mesh_id,
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

    fn submit(&mut self, renderable: &Renderable) {}

    fn execute(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        renderer_state: &mut RendererState,
    ) {
        let read_fbo = resources_manager.borrow_framebuffer(&self.target).unwrap();
        let blit_fbo = resources_manager
            .borrow_framebuffer(&self.blit_buffer)
            .unwrap();

        unsafe {
            self.gl
                .bind_framebuffer(gl::READ_FRAMEBUFFER, Some(read_fbo.handle));

            self.gl
                .bind_framebuffer(gl::DRAW_FRAMEBUFFER, Some(blit_fbo.handle));

            self.gl.blit_framebuffer(
                0,
                0,
                read_fbo.config.width,
                read_fbo.config.height,
                0,
                0,
                blit_fbo.config.width,
                blit_fbo.config.height,
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            )
        }

        renderer_state.set_framebuffer(None, resources_manager);
        unsafe { self.gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }

        renderer_state.set_shader_program(self.shader_id, resources_manager);

        unsafe {
            self.gl.bind_texture(
                gl::TEXTURE_2D,
                Some(
                    resources_manager
                        .borrow_framebuffer(&self.blit_buffer)
                        .unwrap()
                        .get_colour_texture_handle()
                        .unwrap(),
                ),
            );
        }

        upload_draw_data(memory_manager, resources_manager, &self.mesh_id, 1, 0);
        make_draw_call(self.gl, memory_manager, 1);
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
