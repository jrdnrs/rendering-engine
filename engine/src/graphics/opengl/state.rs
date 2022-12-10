use crate::{graphics::state::RasteriserState, platform::rustgl as gl};

#[derive(PartialEq, Clone, Copy)]
pub enum Orientation {
    Front = gl::FRONT as isize,
    Back = gl::BACK as isize,
    FrontAndBack = gl::FRONT_AND_BACK as isize,
}

#[derive(PartialEq, Clone, Copy)]
pub enum VertexWinding {
    Clockwise = gl::CW as isize,
    CounterClockwise = gl::CCW as isize,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Blending {
    One = gl::ONE as isize,
    Zero = gl::ZERO as isize,
    SourceColor = gl::SRC_COLOR as isize,
    DestinationColor = gl::DST_COLOR as isize,
    SourceAlpha = gl::SRC_ALPHA as isize,
    DestinationAlpha = gl::DST_ALPHA as isize,
    OneMinusSourceColor = gl::ONE_MINUS_SRC_COLOR as isize,
    OneMinusDestinationColor = gl::ONE_MINUS_DST_COLOR as isize,
    OneMinusSourceAlpha = gl::ONE_MINUS_SRC_ALPHA as isize,
    OneMinusDestinationAlpha = gl::ONE_MINUS_DST_ALPHA as isize,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Comparison {
    Never = gl::NEVER as isize,
    Always = gl::ALWAYS as isize,
    Less = gl::LESS as isize,
    Greater = gl::GREATER as isize,
    Equal = gl::EQUAL as isize,
    NotEqual = gl::NOTEQUAL as isize,
    EqualOrGreater = gl::GEQUAL as isize,
    EqualOrLess = gl::LEQUAL as isize,
}

impl RasteriserState {
    // set initial settings
    pub fn update_all(&self) {
        if self.blend {
            unsafe { gl::enable(gl::BLEND) }
        } else {
            unsafe { gl::disable(gl::BLEND) }
        }

        if self.depth {
            unsafe { gl::enable(gl::DEPTH_TEST) }
        } else {
            unsafe { gl::disable(gl::DEPTH_TEST) }
        }

        if self.stencil {
            unsafe { gl::enable(gl::STENCIL_TEST) }
        } else {
            unsafe { gl::disable(gl::STENCIL_TEST) }
        }

        if self.scissor {
            unsafe { gl::enable(gl::SCISSOR_TEST) }
        } else {
            unsafe { gl::disable(gl::SCISSOR_TEST) }
        }

        unsafe { gl::blend_func(self.blend_func.0 as u32, self.blend_func.1 as u32) }

        unsafe { gl::depth_func(self.depth_func as u32) }

        unsafe {
            gl::stencil_func(
                self.stencil_func.0 as u32,
                self.stencil_func.1,
                self.stencil_func.2,
            )
        }

        unsafe { gl::depth_mask(self.depth_mask) }

        unsafe { gl::cull_face(self.cull_face as u32) }

        if self.culling {
            unsafe { gl::enable(gl::CULL_FACE) }
        } else {
            unsafe { gl::disable(gl::CULL_FACE) }
        }

        unsafe { gl::front_face(self.front_face as u32) }

        let fill = if self.polygon_mode.1 {
            gl::FILL
        } else {
            gl::LINE
        };
        unsafe {
            gl::polygon_mode(self.polygon_mode.0 as u32, fill);
        }
    }

    pub fn set(&mut self, state: RasteriserState) {
        if self.blend != state.blend {
            self.blend = state.blend;
            if self.blend {
                unsafe { gl::enable(gl::BLEND) }
            } else {
                unsafe { gl::disable(gl::BLEND) }
            }
        }

        if self.depth != state.depth {
            self.depth = state.depth;
            if self.depth {
                unsafe { gl::enable(gl::DEPTH_TEST) }
            } else {
                unsafe { gl::disable(gl::DEPTH_TEST) }
            }
        }

        if self.stencil != state.stencil {
            self.stencil = state.stencil;
            if self.stencil {
                unsafe { gl::enable(gl::STENCIL_TEST) }
            } else {
                unsafe { gl::disable(gl::STENCIL_TEST) }
            }
        }

        if self.scissor != state.scissor {
            self.scissor = state.scissor;
            if self.scissor {
                unsafe { gl::enable(gl::SCISSOR_TEST) }
            } else {
                unsafe { gl::disable(gl::SCISSOR_TEST) }
            }
        }

        if self.blend_func != state.blend_func {
            self.blend_func = state.blend_func;
            unsafe { gl::blend_func(self.blend_func.0 as u32, self.blend_func.1 as u32) }
        }

        if self.depth_func != state.depth_func {
            self.depth_func = state.depth_func;
            unsafe { gl::depth_func(self.depth_func as u32) }
        }

        if self.stencil_func != state.stencil_func {
            self.stencil_func = state.stencil_func;
            unsafe {
                gl::stencil_func(
                    self.stencil_func.0 as u32,
                    self.stencil_func.1,
                    self.stencil_func.2,
                )
            }
        }

        if self.depth_mask != state.depth_mask {
            self.depth_mask = state.depth_mask;
            unsafe { gl::depth_mask(self.depth_mask) }
        }

        if self.cull_face != state.cull_face {
            self.cull_face = state.cull_face;
            unsafe { gl::cull_face(self.cull_face as u32) }
        }

        if self.culling != state.culling {
            self.culling = state.culling;
            if self.culling {
                unsafe { gl::enable(gl::CULL_FACE) }
            } else {
                unsafe { gl::disable(gl::CULL_FACE) }
            }
        }

        if self.front_face != state.front_face {
            self.front_face = state.front_face;
            unsafe { gl::front_face(self.front_face as u32) }
        }

        if self.polygon_mode != state.polygon_mode {
            self.polygon_mode = state.polygon_mode;
            let fill = if self.polygon_mode.1 {
                gl::FILL
            } else {
                gl::LINE
            };
            unsafe {
                gl::polygon_mode(self.polygon_mode.0 as u32, fill);
            }
        }
    }

    pub fn set_blend(&mut self, state: bool) {
        if self.blend != state {
            self.blend = state;
            if self.blend {
                unsafe { gl::enable(gl::BLEND) }
            } else {
                unsafe { gl::disable(gl::BLEND) }
            }
        }
    }

    pub fn set_blend_func(&mut self, source: Blending, destination: Blending) {
        if self.blend_func != (source, destination) {
            self.blend_func = (source, destination);
            unsafe { gl::blend_func(source as u32, destination as u32) }
        }
    }

    pub fn set_depth(&mut self, state: bool) {
        if self.depth != state {
            self.depth = state;
            if self.depth {
                unsafe { gl::enable(gl::DEPTH_TEST) }
            } else {
                unsafe { gl::disable(gl::DEPTH_TEST) }
            }
        }
    }

    pub fn set_stencil(&mut self, state: bool) {
        if self.stencil != state {
            self.stencil = state;
            if self.stencil {
                unsafe { gl::enable(gl::STENCIL_TEST) }
            } else {
                unsafe { gl::disable(gl::STENCIL_TEST) }
            }
        }
    }

    pub fn set_scissor(&mut self, state: bool) {
        if self.scissor != state {
            self.scissor = state;
            if self.scissor {
                unsafe { gl::enable(gl::SCISSOR_TEST) }
            } else {
                unsafe { gl::disable(gl::SCISSOR_TEST) }
            }
        }
    }

    pub fn set_depth_func(&mut self, func: Comparison) {
        if self.depth_func != func {
            self.depth_func = func;
            unsafe { gl::depth_func(func as u32) }
        }
    }

    pub fn set_stencil_func(&mut self, comp: Comparison, reference: i32, mask: u32) {
        if self.stencil_func != (comp, reference, mask) {
            self.stencil_func = (comp, reference, mask);
            unsafe { gl::stencil_func(comp as u32, reference, mask) }
        }
    }

    pub fn set_depth_mask(&mut self, state: bool) {
        if self.depth_mask != state {
            self.depth_mask = state;
            unsafe { gl::depth_mask(state) }
        }
    }
    pub fn set_culled_face(&mut self, face: Orientation) {
        if self.cull_face != face {
            self.cull_face = face;
            unsafe { gl::cull_face(face as u32) }
        }
    }

    pub fn set_culling(&mut self, state: bool) {
        if self.culling != state {
            self.culling = state;
            if self.culling {
                unsafe { gl::enable(gl::CULL_FACE) }
            } else {
                unsafe { gl::disable(gl::CULL_FACE) }
            }
        }
    }

    pub fn set_front_face(&mut self, determinant: VertexWinding) {
        if self.front_face != determinant {
            self.front_face = determinant;
            unsafe { gl::front_face(determinant as u32) }
        }
    }

    pub fn set_polygon_mode(&mut self, sides: Orientation, fill: bool) {
        if self.polygon_mode != (sides, fill) {
            self.polygon_mode = (sides, fill);

            let fill = if self.polygon_mode.1 {
                gl::FILL
            } else {
                gl::LINE
            };
            unsafe {
                gl::polygon_mode(sides as u32, fill);
            }
        }
    }
}
