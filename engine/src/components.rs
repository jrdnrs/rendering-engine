use crate::{
    math::*,
    resource_manager::resource_manager::{MaterialID, MeshID, ShaderProgramID},
};

#[derive(Clone)]
pub struct Renderable {
    pub mesh_id: MeshID,
    pub material_id: MaterialID,
    pub shader_id: ShaderProgramID,
    pub transform: Mat4f,
    pub pipeline_stages: u16,
}

pub struct Block {}

pub struct PointLightBlock {
    // quadratic, linear, constant
    pub attenuation: Vec3f,
}

pub struct SpotLightBlock {
    // quadratic, linear, constant
    pub attenuation: Vec3f,

    pub inner_cutoff_cos: f32, // cosine value
    pub outer_cutoff_cos: f32, // cosine value

    pub direction: Vec3f,
}

pub struct DirLightBlock {
    pub direction: Vec3f,
}
