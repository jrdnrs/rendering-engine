#[cfg(feature = "opengl")]
pub use super::opengl::state::*;

pub struct RasteriserState {
    pub blend: bool,
    pub blend_func: (Blending, Blending),
    pub culling: bool,
    pub cull_face: Orientation,
    pub front_face: VertexWinding,
    pub depth: bool,
    pub depth_mask: bool,
    pub depth_func: Comparison,
    pub stencil: bool,
    pub stencil_func: (Comparison, i32, u32),
    pub scissor: bool,
    pub polygon_mode: (Orientation, bool),
}

impl Default for RasteriserState {
    fn default() -> Self {
        Self {
            blend: true,
            blend_func: (Blending::SourceAlpha, Blending::OneMinusSourceAlpha),
            culling: true,
            cull_face: Orientation::Back,
            front_face: VertexWinding::Clockwise,
            depth: true,
            depth_mask: true,
            depth_func: Comparison::Less,
            stencil: false,
            stencil_func: (Comparison::Always, 0, u32::max_value()),
            scissor: false,
            polygon_mode: (Orientation::FrontAndBack, true),
        }
    }
}
