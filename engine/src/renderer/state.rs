use glow::{self as gl, HasContext};

use crate::{
    math::math::*,
    memory_manager::{memory_manager::MemoryManager, uniform_layouts::Light},
    resource_manager::resource_manager::{
        FramebufferID, ResourceIDTrait, ResourceManagerTrait, ResourcesManager, ShaderProgramID,
    },
};

#[derive(PartialEq)]
pub struct RasteriserState {
    pub blend: bool,
    pub blend_func: (u32, u32),
    pub culling: bool,
    pub cull_face: u32,
    pub front_face: u32,
    pub depth: bool,
    pub depth_mask: bool,
    pub depth_func: u32,
    pub stencil: bool,
    pub stencil_func: (u32, i32, u32),
    pub scissor: bool,
}

impl Default for RasteriserState {
    fn default() -> Self {
        Self {
            blend: true,
            blend_func: (gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA),
            culling: true,
            cull_face: gl::BACK,
            front_face: gl::CW,
            depth: true,
            depth_mask: true,
            depth_func: gl::LESS,
            stencil: false,
            stencil_func: (gl::ALWAYS, 0, u32::max_value()),
            scissor: false,
        }
    }
}

pub struct RendererState<'a> {
    gl: &'a gl::Context,

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

    pub lights: Vec<Light>,

    pub rasteriser_state: RasteriserState,
}

impl<'a> RendererState<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        let rs = Self {
            gl,

            shader_program: None,
            framebuffer: None,

            view_transform: Mat4f::identity(),
            projection_transform: Mat4f::identity(),
            camera_position: Vec3f::new(0.0, 0.0, 0.0),
            camera_direction: Vec3f::new(0.0, 0.0, 0.0),
            lights: Vec::new(),

            rasteriser_state: RasteriserState::default(),
        };

        rs.init();

        rs
    }

    // set initial settings
    fn init(&self) {
        if self.rasteriser_state.blend {
            unsafe { self.gl.enable(gl::BLEND) }
        } else {
            unsafe { self.gl.disable(gl::BLEND) }
        }

        if self.rasteriser_state.depth {
            unsafe { self.gl.enable(gl::DEPTH_TEST) }
        } else {
            unsafe { self.gl.disable(gl::DEPTH_TEST) }
        }

        if self.rasteriser_state.stencil {
            unsafe { self.gl.enable(gl::STENCIL_TEST) }
        } else {
            unsafe { self.gl.disable(gl::STENCIL_TEST) }
        }

        if self.rasteriser_state.scissor {
            unsafe { self.gl.enable(gl::SCISSOR_TEST) }
        } else {
            unsafe { self.gl.disable(gl::SCISSOR_TEST) }
        }

        unsafe {
            self.gl.blend_func(
                self.rasteriser_state.blend_func.0,
                self.rasteriser_state.blend_func.1,
            )
        }

        unsafe { self.gl.depth_func(self.rasteriser_state.depth_func) }

        unsafe {
            self.gl.stencil_func(
                self.rasteriser_state.stencil_func.0,
                self.rasteriser_state.stencil_func.1,
                self.rasteriser_state.stencil_func.2,
            )
        }

        unsafe { self.gl.depth_mask(self.rasteriser_state.depth_mask) }

        unsafe { self.gl.cull_face(self.rasteriser_state.cull_face) }

        unsafe { self.gl.front_face(self.rasteriser_state.front_face) }
    }

    pub fn upload_camera_data(&self, memory_manager: &mut MemoryManager) {
        memory_manager.set_projection_matrix(self.projection_transform.transpose());
        memory_manager.set_view_matrix(self.view_transform.transpose());
        memory_manager.set_camera_direction(self.camera_direction);
        memory_manager.set_camera_position(self.camera_position);
    }

    pub fn upload_light_data(&self, memory_manager: &mut MemoryManager) {
        memory_manager.set_lights_data(&self.lights);
        memory_manager.set_light_count(self.lights.len() as u32);
    }

    pub fn set_rasteriser_state(&mut self, state: RasteriserState) {
        if self.rasteriser_state == state {
            return;
        }

        if state.blend != self.rasteriser_state.blend {
            self.rasteriser_state.blend = state.blend;
            if self.rasteriser_state.blend {
                unsafe { self.gl.enable(gl::BLEND) }
            } else {
                unsafe { self.gl.disable(gl::BLEND) }
            }
        }

        if state.depth != self.rasteriser_state.depth {
            self.rasteriser_state.depth = state.depth;
            if self.rasteriser_state.depth {
                unsafe { self.gl.enable(gl::DEPTH_TEST) }
            } else {
                unsafe { self.gl.disable(gl::DEPTH_TEST) }
            }
        }

        if state.stencil != self.rasteriser_state.stencil {
            self.rasteriser_state.stencil = state.stencil;
            if self.rasteriser_state.stencil {
                unsafe { self.gl.enable(gl::STENCIL_TEST) }
            } else {
                unsafe { self.gl.disable(gl::STENCIL_TEST) }
            }
        }

        if state.scissor != self.rasteriser_state.scissor {
            self.rasteriser_state.scissor = state.scissor;
            if self.rasteriser_state.scissor {
                unsafe { self.gl.enable(gl::SCISSOR_TEST) }
            } else {
                unsafe { self.gl.disable(gl::SCISSOR_TEST) }
            }
        }

        if state.blend_func != self.rasteriser_state.blend_func {
            self.rasteriser_state.blend_func = state.blend_func;
            unsafe {
                self.gl.blend_func(
                    self.rasteriser_state.blend_func.0,
                    self.rasteriser_state.blend_func.1,
                )
            }
        }

        if state.depth_func != self.rasteriser_state.depth_func {
            self.rasteriser_state.depth_func = state.depth_func;
            unsafe { self.gl.depth_func(self.rasteriser_state.depth_func) }
        }

        if state.stencil_func != self.rasteriser_state.stencil_func {
            self.rasteriser_state.stencil_func = state.stencil_func;
            unsafe {
                self.gl.stencil_func(
                    self.rasteriser_state.stencil_func.0,
                    self.rasteriser_state.stencil_func.1,
                    self.rasteriser_state.stencil_func.2,
                )
            }
        }

        if state.depth_mask != self.rasteriser_state.depth_mask {
            self.rasteriser_state.depth_mask = state.depth_mask;
            unsafe { self.gl.depth_mask(self.rasteriser_state.depth_mask) }
        }

        if state.cull_face != self.rasteriser_state.cull_face {
            self.rasteriser_state.cull_face = state.cull_face;
            unsafe { self.gl.cull_face(self.rasteriser_state.cull_face) }
        }

        if state.front_face != self.rasteriser_state.front_face {
            self.rasteriser_state.front_face = state.front_face;
            unsafe { self.gl.front_face(self.rasteriser_state.front_face) }
        }
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
        framebuffer_id: Option<FramebufferID>,
        resources_manager: &ResourcesManager,
    ) -> bool {
        if let Some(new_framebuffer_id) = framebuffer_id {
            if let Some(current_framebuffer_id) = self.framebuffer {
                if current_framebuffer_id == new_framebuffer_id {
                    return false;
                }
            }

            self.framebuffer = Some(new_framebuffer_id);
            resources_manager
                .framebuffer_manager
                .borrow(&new_framebuffer_id)
                .unwrap()
                .bind();

            true
        } else {
            self.framebuffer = None;
            unsafe { self.gl.bind_framebuffer(gl::FRAMEBUFFER, None) }
            true
        }
    }
}
