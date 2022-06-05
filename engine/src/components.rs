use crate::{
    math::math::*,
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

pub struct LightBlock {}
