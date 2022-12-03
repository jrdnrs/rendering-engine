use std::collections::HashMap;

use super::{ApiEnum, ApiHandle};
use crate::math::Mat4f;
pub struct Program {
    pub handle: ApiHandle,
    pub shader_handles: Vec<ApiHandle>,
    pub uniform_loc_cache: HashMap<String, ApiEnum>,
    pub shaders_path: &'static str,
}

pub enum ShaderDataType {
    Uint1,
    Uint2,
    Uint3,
    Uint4,
    Int1,
    Int2,
    Int3,
    Int4,
    Float1,
    Float2,
    Float3,
    Float4,
    Mat2f,
    Mat3f,
    Mat4f,
}

pub enum ShaderData<'a> {
    Uint1(u32),
    Uint2(u32, u32),
    Uint3(u32, u32, u32),
    Uint4(u32, u32, u32, u32),
    Int1(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),
    Float1(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Mat2f(),
    Mat3f(),
    Mat4f(&'a Mat4f),
}
