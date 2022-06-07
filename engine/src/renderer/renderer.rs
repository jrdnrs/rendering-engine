use glow::{self as gl, HasContext};
use log::error;

use super::{
    camera::Camera,
    pipeline::RendererPipeline,
    pipeline_stages::{
        debug::DebugStage, post_process::PostProcessStage, scene::SceneStage, sky::SkyStage, *,
    },
    state::RendererState,
};
use crate::{
    components::Renderable,
    math::math::*,
    memory_manager::{
        memory_manager::MemoryManager,
        uniform_layouts::{self, Light},
    },
    resource_manager::{
        framebuffer::{Framebuffer, FramebufferAttachment, FramebufferConfig},
        model,
        resource_manager::{
            MaterialID, MeshID, ResourceIDTrait, ResourcesManager, ShaderProgramID, TextureID,
        },
        texture::TextureConfig,
    },
};

pub struct Renderer<'a> {
    pub gl: &'a gl::Context,
    pub resources_manager: ResourcesManager<'a>,
    pub memory_manager: MemoryManager<'a>,
    pub renderer_state: RendererState<'a>,
    pub renderer_pipeline: RendererPipeline<'a>,
    pub camera: Camera,
}

impl<'a> Renderer<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        let mut r = Renderer {
            gl,
            resources_manager: ResourcesManager::new(gl),
            memory_manager: MemoryManager::new(gl),
            renderer_state: RendererState::new(gl),
            renderer_pipeline: RendererPipeline::new(gl),
            camera: Camera::new_perspective(70.0, 0.1, 100.0),
        };
        r.init();

        r
    }

    fn init(&mut self) {
        unsafe {
            self.gl.debug_message_callback(|_, _, _, _, msg: &str| {
                error!("{}", msg);
            });
        }

        let config = FramebufferConfig {
            colour: FramebufferAttachment::Texture {
                internal_format: gl::RGBA16F,
            },
            depth: FramebufferAttachment::Renderbuffer {
                internal_format: gl::DEPTH_COMPONENT,
            },
            stencil: FramebufferAttachment::None,
            width: crate::WIDTH,
            height: crate::HEIGHT,
            samples: 8,
        };

        let fb_id = self.resources_manager.load_framebuffer(&config, true);

        self.renderer_pipeline
            .add_stage(SceneStage::new(self.gl, fb_id), STAGE_SCENE);
        self.renderer_pipeline
            .add_stage(SkyStage::new(self.gl, fb_id), STAGE_SKY);
        self.renderer_pipeline
            .add_stage(DebugStage::new(self.gl, fb_id), STAGE_DEBUG);
        self.renderer_pipeline.add_stage(
            PostProcessStage::new(self.gl, fb_id, &mut self.resources_manager),
            STAGE_POST_PROCESS,
        );
    }

    pub fn set_viewport(&mut self, width: i32, height: i32) {
        self.camera.update_projection(width as f32, height as f32);

        for i in 0..self.resources_manager.resize_framebuffers.len() {
            let id = self.resources_manager.resize_framebuffers[i];

            if let Some(framebuffer) = self.resources_manager.borrow_mut_framebuffer(&id) {
                if framebuffer.config.width != width || framebuffer.config.height != height {
                    framebuffer.resize(width, height)
                }
            }
        }

        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn set_clear_colour(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.gl.clear_color(r, g, b, a) }
    }

    pub fn clear(&self) {
        unsafe { self.gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }
    }

    pub fn add_light(&mut self, light: Light) {
        self.renderer_state.lights.push(light);
    }

    pub fn begin(&mut self) {
        self.camera.update_view();
        self.renderer_state.view_transform = self.camera.view.clone();
        self.renderer_state.projection_transform = self.camera.projection.clone();
        self.renderer_state.camera_position = self.camera.position;
        self.renderer_state.camera_direction = self.camera.direction;
        self.memory_manager.wait_for_section_lock();
    }

    pub fn end(&mut self) {
        self.renderer_pipeline.execute(
            &mut self.memory_manager,
            &mut self.resources_manager,
            &mut self.renderer_state,
        );
        self.memory_manager.set_section_lock();
        self.memory_manager.advance_sections();
        self.renderer_state.lights.clear();
    }

    pub fn draw(&mut self, renderable: &Renderable) {
        self.renderer_pipeline.submit(renderable)
    }

    pub fn load_shader(&mut self, path: &'static str) -> ShaderProgramID {
        self.resources_manager.load_shader(path)
    }

    pub fn load_mesh(&mut self, mesh: model::Mesh) -> MeshID {
        self.resources_manager.load_mesh(mesh)
    }

    pub fn load_material(&mut self, material: model::Material) -> MaterialID {
        let id = self.resources_manager.load_material(material);
        let index = id.index() as usize;
        let material = &self.resources_manager.material_manager.resources[index];

        let diff_texture_handle = unsafe {
            self.gl.get_texture_handle(
                self.resources_manager.texture_manager.resources
                    [material.diffuse_texture_id.index() as usize]
                    .handle,
            )
        };

        unsafe { self.gl.make_texture_handle_resident(diff_texture_handle) }

        let material_uniform = uniform_layouts::Material {
            shininess: material.shininess,
            diffuse_texture: Vec2u::new(
                diff_texture_handle.0.get() as u32,
                (diff_texture_handle.0.get() >> 32) as u32,
            ),
            // diffuse_texture: Vec2u::new(
            //     0,
            //     0,
            // ),
            ..Default::default()
        };

        self.memory_manager
            .set_material_data(material_uniform, index);

        id
    }

    pub fn load_texture(
        &mut self,
        path: &'static str,
        config: &TextureConfig,
    ) -> Result<TextureID, String> {
        self.resources_manager.load_texture(path, config)
    }

    pub fn load_skybox_textures(
        &mut self,
        paths: [&'static str; 6],
        config: &TextureConfig,
    ) -> Result<TextureID, String> {
        self.resources_manager.load_skybox_textures(paths, config)
    }
}
