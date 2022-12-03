use glow::{self as gl, HasContext};

use super::PipelineStage;
use crate::{
    components::Renderable,
    math::Vec4f,
    memory_manager::{
        memory_manager::{DrawElementsIndirectCommand, MemoryManager, DRAW_COMMAND_SIZE},
        uniform_layouts::{
            GeneralPurposeIndexStorageBuffer, GeneralPurposeStorageBuffer,
            GeneralPurposeVecStorageBuffer,
        },
    },
    renderer::state::{RasteriserState, RendererState},
    resource_manager::{
        framebuffer::FramebufferAttachment,
        resource_manager::{FramebufferID, MeshID, ResourcesManager, ShaderProgramID},
    },
};

pub struct BloomStage<'a> {
    gl: &'a gl::Context,
    target: FramebufferID,
    upsample_shader_id: ShaderProgramID,
    downsample_shader_id: ShaderProgramID,
}

impl<'a> BloomStage<'a> {
    pub fn new(
        gl: &'a gl::Context,
        target: FramebufferID,
        resources_manager: &mut ResourcesManager<'a>,
    ) -> Self {
        let downsample_shader_id =
            resources_manager.load_shader("res/shaders/bloom_downsample.glsl");
        let upsample_shader_id = resources_manager.load_shader("res/shaders/bloom_upsample.glsl");

        Self {
            gl,
            target,
            upsample_shader_id,
            downsample_shader_id,
        }
    }
}

impl<'a> PipelineStage for BloomStage<'a> {
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
        renderables: &[Renderable],
    ) {
        let fb = resources_manager.borrow_framebuffer(&self.target).unwrap();
        let texture = fb.get_colour_texture_handle().unwrap();

        unsafe {
            self.gl.active_texture(gl::TEXTURE0);
            self.gl.bind_texture(gl::TEXTURE_2D, Some(texture));
        }

        if let FramebufferAttachment::Texture { levels, .. } = fb.config.colour {
            renderer_state.set_shader_program(self.downsample_shader_id, resources_manager);
            for level in 0..(levels - 1) {
                self.downsample(memory_manager, resources_manager, level);
            }

            renderer_state.set_shader_program(self.upsample_shader_id, resources_manager);
            for level in (1..levels).rev() {
                self.upsample(memory_manager, resources_manager, level);
            }
        }
    }
}

impl BloomStage<'_> {
    fn downsample(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        read_mip: u32,
    ) {
        let fb = resources_manager.borrow_framebuffer(&self.target).unwrap();
        let texture = fb.get_colour_texture_handle().unwrap();

        // 64 threads per subgroup, one per written pixel, so calculate number of subgroups based on resolution of
        // write mip level which is one higher

        let write_w = fb.config.width / 2u32.pow(read_mip + 1);
        let write_h = fb.config.height / 2u32.pow(read_mip + 1);

        let read_w = write_w * 2;
        let read_h = write_h * 2;

        let blocks_w = (write_w + 15) / 16;
        let blocks_h = (write_h + 15) / 16;

        memory_manager.reserve_per_draw_shader_data(1);
        memory_manager.set_general(GeneralPurposeStorageBuffer {
            indices: GeneralPurposeIndexStorageBuffer {
                index_1: read_mip,
                ..Default::default()
            },
            vecs: GeneralPurposeVecStorageBuffer {
                vec_1: Vec4f::new(write_w as f32, write_h as f32, 0.0, 0.0),
                ..Default::default()
            },
        });

        unsafe {
            self.gl.bind_image_texture(
                1,
                texture,
                (read_mip + 1) as i32,
                false,
                0,
                gl::WRITE_ONLY,
                gl::RGBA16F,
            );

            self.gl.dispatch_compute(blocks_w, blocks_h, 1);
            self.gl.memory_barrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }

    fn upsample(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        read_mip: u32,
    ) {
        let fb = resources_manager.borrow_framebuffer(&self.target).unwrap();
        let texture = fb.get_colour_texture_handle().unwrap();

        // 64 threads per subgroup, one per written pixel, so calculate number of subgroups based on resolution of
        // write mip level which is one lower

        let read_w = fb.config.width / 2u32.pow(read_mip);
        let read_h = fb.config.height / 2u32.pow(read_mip);

        let write_w = read_w * 2;
        let write_h = read_h * 2;

        let blocks_w = (write_w + 15) / 16;
        let blocks_h = (write_h + 15) / 16;

        memory_manager.reserve_per_draw_shader_data(1);
        memory_manager.set_general(GeneralPurposeStorageBuffer {
            indices: GeneralPurposeIndexStorageBuffer {
                index_1: read_mip,
                ..Default::default()
            },
            vecs: GeneralPurposeVecStorageBuffer {
                vec_1: Vec4f::new(write_w as f32, write_h as f32, 0.0, 0.0),
                ..Default::default()
            },
        });

        unsafe {
            self.gl.bind_image_texture(
                1,
                texture,
                (read_mip - 1) as i32,
                false,
                0,
                gl::READ_WRITE,
                gl::RGBA16F,
            );

            self.gl.dispatch_compute(blocks_w, blocks_h, 1);
            self.gl.memory_barrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }
}
