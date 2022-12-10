use super::{
    camera::Camera,
    pipeline::RendererPipeline,
    pipeline_stages::{
        bloom::BloomStage, debug::DebugStage, post_process::PostProcessStage, scene::SceneStage,
        shadow::ShadowStage, sky::SkyStage, *,
    },
    state::RendererState,
};
use crate::{
    components::Renderable,
    graphics::{
        self,
        framebuffer::{FramebufferAttachmentConfig, FramebufferConfig, InternalFormat},
        texture::{TextureConfig, TextureType}, state::RasteriserState,
    },
    math::*,
    memory_manager::{
        memory_manager::MemoryManager,
        uniform_layouts::{self, DirectionalLight, PointLight, SpotLight},
    },
    resource_manager::{
        model,
        resource_manager::{
            MaterialID, MeshID, ResourceIDTrait, ResourcesManager, ShaderProgramID, TextureID,
        },
    },
};

pub struct Renderer<'a> {
    pub renderer_state: RendererState,
    pub rasteriser_state: RasteriserState,
    pub renderer_pipeline: RendererPipeline<'a>,
    pub resources_manager: ResourcesManager,
    pub memory_manager: MemoryManager,
    pub camera: Camera,

    renderables: Vec<Renderable>,
}

impl Renderer<'_> {
    pub fn new() -> Self {
        let mut r = Renderer {
            renderer_state: RendererState::new(),
            rasteriser_state: RasteriserState::default(),
            renderer_pipeline: RendererPipeline::new(),
            resources_manager: ResourcesManager::new(),
            memory_manager: MemoryManager::new(),
            camera: Camera::new_perspective(70.0, 0.1, 100.0),
            renderables: Vec::new(),
        };
        r.init();
        r
    }

    fn init(&mut self) {
        self.rasteriser_state.update_all();

        let config = FramebufferConfig {
            colour: FramebufferAttachmentConfig::Texture {
                target: TextureType::T2D,
                internal_format: InternalFormat::RGBA16F,
                layers: 1,
                levels: 5,
            },
            depth: FramebufferAttachmentConfig::Texture {
                target: TextureType::T2D,
                internal_format: InternalFormat::Depth32F,
                layers: 1,
                levels: 1,
            },
            stencil: FramebufferAttachmentConfig::None,
            width: crate::WIDTH,
            height: crate::HEIGHT,
            samples: crate::SAMPLES,
        };

        let fb_id = self.resources_manager.load_framebuffer(&config, true);

        self.renderer_pipeline.add_stage(
            ShadowStage::new(
                fb_id,
                &mut self.memory_manager,
                &mut self.resources_manager,
                &mut self.renderer_state,
            ),
            STAGE_SHADOW,
        );
        self.renderer_pipeline
            .add_stage(SceneStage::new(fb_id), STAGE_SCENE);
        self.renderer_pipeline
            .add_stage(SkyStage::new(fb_id), STAGE_SKY);
        self.renderer_pipeline.add_stage(
            BloomStage::new(fb_id, &mut self.resources_manager),
            STAGE_BLOOM,
        );
        self.renderer_pipeline.add_stage(
            DebugStage::new(fb_id, &mut self.resources_manager),
            STAGE_DEBUG,
        );
        self.renderer_pipeline.add_stage(
            PostProcessStage::new(fb_id, &mut self.resources_manager),
            STAGE_POST_PROCESS,
        );
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.camera.update_projection(width as f32, height as f32);

        for i in 0..self.resources_manager.resize_framebuffers.len() {
            let id = self.resources_manager.resize_framebuffers[i];

            if let Some(framebuffer) = self.resources_manager.borrow_mut_framebuffer(&id) {
                if framebuffer.config.width != width || framebuffer.config.height != height {
                    framebuffer.resize(width, height)
                }
            }
        }

        graphics::set_viewport(0, 0, width, height);
    }

    pub fn add_point_light(&mut self, mut light: PointLight) {
        light.views[0] = self.renderer_state.light_persp_projection
            * Camera::look_at(
                &light.position,
                &Vec3f::new(-1.0, 0.0, 0.0),
                &Vec3f::new(0.0, -1.0, 0.0),
            );
        light.views[1] = self.renderer_state.light_persp_projection
            * Camera::look_at(
                &light.position,
                &Vec3f::new(1.0, 0.0, 0.0),
                &Vec3f::new(0.0, -1.0, 0.0),
            );
        light.views[2] = self.renderer_state.light_persp_projection
            * Camera::look_at(
                &light.position,
                &Vec3f::new(0.0, -1.0, 0.0),
                &Vec3f::new(0.0, 0.0, 1.0),
            );
        light.views[3] = self.renderer_state.light_persp_projection
            * Camera::look_at(
                &light.position,
                &Vec3f::new(0.0, 1.0, 0.0),
                &Vec3f::new(0.0, 0.0, -1.0),
            );
        light.views[4] = self.renderer_state.light_persp_projection
            * Camera::look_at(
                &light.position,
                &Vec3f::new(0.0, 0.0, -1.0),
                &Vec3f::new(0.0, -1.0, 0.0),
            );
        light.views[5] = self.renderer_state.light_persp_projection
            * Camera::look_at(
                &light.position,
                &Vec3f::new(0.0, 0.0, 1.0),
                &Vec3f::new(0.0, -1.0, 0.0),
            );

        self.renderer_state.point_lights.push(light);
    }

    pub fn add_spot_light(&mut self, mut light: SpotLight) {
        light.view = self.renderer_state.light_persp_projection
            * Camera::look_at(
                &light.position,
                &light.direction,
                &Vec3f::new(0.0, 1.0, 0.0),
            );
        self.renderer_state.spot_lights.push(light);
    }

    pub fn set_directional_light(&mut self, mut light: DirectionalLight) {
        light.view = self.renderer_state.light_ortho_projection
            * Camera::look_at(
                &light.position,
                &light.direction,
                &Vec3f::new(0.0, 1.0, 0.0),
            );
        self.renderer_state.directional_light = Some(light);
    }

    pub fn begin(&mut self) {
        self.camera.update_view();
        self.renderer_state.view_transform = self.camera.view.clone();
        self.renderer_state.projection_transform = self.camera.projection.clone();
        self.renderer_state.camera_position = self.camera.position;
        self.renderer_state.camera_direction = self.camera.direction;
    }

    pub fn end(&mut self) {
        self.memory_manager.wait_for_section_lock();
        self.renderer_state
            .upload_camera_data(&mut self.memory_manager);
        self.renderer_state
            .upload_light_data(&mut self.memory_manager);
        self.renderer_pipeline.execute(
            &mut self.memory_manager,
            &mut self.resources_manager,
            &mut self.renderer_state,
            &mut self.rasteriser_state,
            &self.renderables,
        );
        self.memory_manager.set_section_lock();

        self.memory_manager.advance_sections();
        self.renderer_state.reset_lights();
        self.renderables.clear();
    }

    pub fn draw(&mut self, renderable: &Renderable) {
        self.renderables.push(renderable.clone());
        self.renderer_pipeline
            .submit(self.renderables.len() - 1, renderable.pipeline_stages);
    }

    pub fn load_shader(&mut self, path: &'static str) -> ShaderProgramID {
        self.resources_manager.load_shader(path)
    }

    pub fn load_mesh(&mut self, mesh: model::Mesh) -> MeshID {
        self.resources_manager.load_mesh(mesh)
    }

    pub fn load_material(&mut self, material: model::Material) -> MaterialID {
        let id = self.resources_manager.load_material(material);
        let index = id.index();
        let material = self.resources_manager.material_manager.resources[index as usize];

        let diff_texture_handle;
        let spec_texture_handle;
        let norm_texture_handle;

        if let Some(texture_id) = material.diffuse_texture_id {
            let texture = self
                .resources_manager
                .borrow_mut_texture(&texture_id)
                .unwrap();
            texture.make_texture_resident();
            diff_texture_handle = texture.shader_texture_handle.unwrap();
        } else {
            diff_texture_handle = self
                .resources_manager
                .borrow_texture(&self.resources_manager.placeholder_diffuse_texture)
                .unwrap()
                .shader_texture_handle
                .unwrap();
        }

        if let Some(texture_id) = material.specular_texture_id {
            let texture = self
                .resources_manager
                .borrow_mut_texture(&texture_id)
                .unwrap();
            texture.make_texture_resident();
            spec_texture_handle = texture.shader_texture_handle.unwrap();
        } else {
            spec_texture_handle = self
                .resources_manager
                .borrow_texture(&self.resources_manager.placeholder_specular_texture)
                .unwrap()
                .shader_texture_handle
                .unwrap();
        }

        if let Some(texture_id) = material.normal_texture_id {
            let texture = self
                .resources_manager
                .borrow_mut_texture(&texture_id)
                .unwrap();
            texture.make_texture_resident();
            norm_texture_handle = texture.shader_texture_handle.unwrap();
        } else {
            norm_texture_handle = self
                .resources_manager
                .borrow_texture(&self.resources_manager.placeholder_normal_texture)
                .unwrap()
                .shader_texture_handle
                .unwrap();
        }

        let material_uniform = uniform_layouts::Material {
            shininess: material.shininess,
            diffuse_texture: Vec2u::new(
                diff_texture_handle as u32,
                (diff_texture_handle >> 32) as u32,
            ),
            specular_texture: Vec2u::new(
                spec_texture_handle as u32,
                (spec_texture_handle >> 32) as u32,
            ),
            normal_texture: Vec2u::new(
                norm_texture_handle as u32,
                (norm_texture_handle >> 32) as u32,
            ),
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
