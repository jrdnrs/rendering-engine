use crate::{math::*, renderer::asset::{MeshID, MaterialID, ShaderProgramID}};

#[derive(Clone)]
pub struct Renderable {
    pub mesh_id: MeshID,
    pub material_id: MaterialID,
    pub shader_id: ShaderProgramID,
    pub transform: Mat4f,
}

pub struct Block {}

pub struct LightBlock {}
