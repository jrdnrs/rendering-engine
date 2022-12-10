use crate::{
    math::*,
    memory_manager::{
        memory_manager::MemoryManager,
        uniform_layouts::{DirectionalLight, PointLight, SpotLight},
    },
    resource_manager::resource_manager::{
        FramebufferID, ResourceIDTrait, ResourceManagerTrait, ResourcesManager, ShaderProgramID,
    }, graphics::framebuffer::Framebuffer,
};

pub struct RendererState {
    pub shader_program: Option<ShaderProgramID>,
    pub framebuffer: Option<FramebufferID>,
    // pub vertex_array: ,
    // pub index_buffer: ,
    // pub vertex_buffer: ,
    // pub indirect_draw_buffer: ,
    pub view_transform: Mat4f,
    pub projection_transform: Mat4f,
    pub camera_position: Vec3f,
    pub camera_direction: Vec3f,

    pub point_lights: Vec<PointLight>,
    pub spot_lights: Vec<SpotLight>,
    pub directional_light: Option<DirectionalLight>,

    pub light_ortho_projection: Mat4f,
    pub light_persp_projection: Mat4f,
}

impl RendererState {
    pub fn new() -> Self {
        Self {
            shader_program: None,
            framebuffer: None,

            view_transform: Mat4f::identity(),
            projection_transform: Mat4f::identity(),
            camera_position: Vec3f::new(0.0, 0.0, 0.0),
            camera_direction: Vec3f::new(0.0, 0.0, 0.0),

            point_lights: Vec::new(),
            spot_lights: Vec::new(),
            directional_light: None,

            light_ortho_projection: Mat4f::identity(),
            light_persp_projection: Mat4f::identity(),
        }
    }

    pub fn upload_camera_data(&self, memory_manager: &mut MemoryManager) {
        memory_manager.set_projection_matrix(self.projection_transform);
        memory_manager.set_view_matrix(self.view_transform);
        memory_manager.set_camera_direction(self.camera_direction);
        memory_manager.set_camera_position(self.camera_position);
    }

    pub fn upload_light_data(&self, memory_manager: &mut MemoryManager) {
        memory_manager.set_point_light_data_slice(&self.point_lights);
        memory_manager.set_point_light_count(self.point_lights.len() as u32);

        memory_manager.set_spot_light_data_slice(&self.spot_lights);
        memory_manager.set_spot_light_count(self.spot_lights.len() as u32);

        if let Some(directional_light) = self.directional_light {
            memory_manager.set_directional_light_data(directional_light);
            memory_manager.set_directional_light_count(1);
        } else {
            memory_manager.set_directional_light_count(0);
        }
    }

    pub fn reset_lights(&mut self) {
        self.point_lights.clear();
        self.spot_lights.clear();
        self.directional_light = None;
    }

    pub fn set_shader_program(
        &mut self,
        shader_program_id: ShaderProgramID,
        resources_manager: &ResourcesManager,
    ) -> bool {
        if let Some(current_shader_id) = self.shader_program {
            if current_shader_id == shader_program_id {
                return false;
            }
        }

        self.shader_program = Some(shader_program_id);
        resources_manager
            .shader_program_manager
            .borrow(&shader_program_id)
            .unwrap()
            .bind();

        true
    }

    pub fn set_framebuffer(
        &mut self,
        framebuffer_id: Option<&FramebufferID>,
        resources_manager: &ResourcesManager,
    ) -> bool {
        if let Some(new_framebuffer_id) = framebuffer_id {
            if let Some(current_framebuffer_id) = self.framebuffer {
                if current_framebuffer_id == *new_framebuffer_id {
                    return false;
                }
            }

            self.framebuffer = Some(new_framebuffer_id.clone());
            resources_manager
                .framebuffer_manager
                .borrow(new_framebuffer_id)
                .unwrap()
                .bind();

            true
        } else {
            self.framebuffer = None;
            Framebuffer::unbind();
            true
        }
    }
}
