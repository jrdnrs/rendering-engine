use super::PipelineStage;
use crate::{
    components::Renderable,
    graphics::{
        self,
        framebuffer::{FramebufferAttachment, FramebufferAttachmentConfig, InternalFormat},
        shader::Program,
        state::RasteriserState,
        AccessModifier, Barriers,
    },
    math::Vec4f,
    memory_manager::{
        memory_manager::MemoryManager,
        uniform_layouts::{
            GeneralPurposeIndexStorageBuffer, GeneralPurposeStorageBuffer,
            GeneralPurposeVecStorageBuffer,
        },
    },
    platform::rustgl,
    renderer::state::RendererState,
    resource_manager::resource_manager::{FramebufferID, ResourcesManager, ShaderProgramID},
};

pub struct BloomStage {
    target: FramebufferID,
    upsample_shader_id: ShaderProgramID,
    downsample_shader_id: ShaderProgramID,
}

impl BloomStage {
    pub fn new(target: FramebufferID, resources_manager: &mut ResourcesManager) -> Self {
        let downsample_shader_id =
            resources_manager.load_shader("res/shaders/bloom_downsample.glsl");
        let upsample_shader_id = resources_manager.load_shader("res/shaders/bloom_upsample.glsl");

        Self {
            target,
            upsample_shader_id,
            downsample_shader_id,
        }
    }
}

impl PipelineStage for BloomStage {
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

        if let FramebufferAttachment::Texture(texture) = &fb.colour_handle {
            unsafe {
                rustgl::active_texture(rustgl::TEXTURE0);
            }
            texture.bind()
        }

        if let FramebufferAttachmentConfig::Texture { levels, .. } = fb.config.colour {
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

impl BloomStage {
    fn downsample(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        read_mip: u32,
    ) {
        let fb = resources_manager.borrow_framebuffer(&self.target).unwrap();

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

        if let FramebufferAttachment::Texture(ref texture) = fb.colour_handle {
            texture.bind_image_unit(
                1,
                read_mip + 1,
                0,
                AccessModifier::WriteOnly,
                InternalFormat::RGBA16F,
            )
        }

        Program::dispatch_compute(blocks_w, blocks_h, 1);
        graphics::memory_barrier(Barriers::ShaderImageAccess as u32);
    }

    fn upsample(
        &mut self,
        memory_manager: &mut MemoryManager,
        resources_manager: &mut ResourcesManager,
        read_mip: u32,
    ) {
        let fb = resources_manager.borrow_framebuffer(&self.target).unwrap();

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

        if let FramebufferAttachment::Texture(ref texture) = fb.colour_handle {
            texture.bind_image_unit(
                1,
                read_mip - 1,
                0,
                AccessModifier::ReadWrite,
                InternalFormat::RGBA16F,
            )
        }

        Program::dispatch_compute(blocks_w, blocks_h, 1);
        graphics::memory_barrier(Barriers::ShaderImageAccess as u32);
    }
}
