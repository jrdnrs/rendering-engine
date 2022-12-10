use bytemuck::{Pod, Zeroable};

use super::resource_manager::TextureID;
use crate::math::*;

pub const VERTEX_SIZE: u32 = std::mem::size_of::<Vertex>() as u32;

pub struct Model {
    pub meshes: Vec<Mesh>,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub shininess: f32,
    pub diffuse_texture_id: Option<TextureID>,
    pub specular_texture_id: Option<TextureID>,
    pub normal_texture_id: Option<TextureID>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3f,
    pub normal: Vec3f,
    pub tangent: Vec3f,
    pub colour: Vec4f,
    pub tex_coord: Vec2f,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}
