use bytemuck::{Pod, Zeroable};

use super::resource_manager::TextureID;
use crate::math::math::*;

pub const VERTEX_SIZE: u32 = std::mem::size_of::<Vertex>() as u32;

pub struct Model {
    pub meshes: Vec<Mesh>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub struct Material {
    pub shininess: f32,
    pub diffuse_texture_id: TextureID,
    pub specular_texture_id: TextureID,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3f,
    pub normal: Vec3f,
    pub colour: Vec4f,
    pub tex_coord: Vec2f,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

mod tests {
    use super::*;
    use crate::math::*;

    #[test]
    fn vertex_slice() {
        let mut vertices_vec: Vec<Vertex> = Vec::new();
        let mut control_vec: Vec<f32> = Vec::new();

        for i in 0..10 {
            let n = i as f32;
            vertices_vec.push(Vertex {
                position: Vec3f::new(n, n, n),
                normal: Vec3f::new(2.0 * n, 2.0 * n, 2.0 * n),
                colour: Vec4f::new(3.0 * n, 3.0 * n, 3.0 * n, 3.0 * n),
                tex_coord: Vec2f::new(4.0 * n, 4.0 * n),
            });

            control_vec.extend_from_slice(&[
                n,
                n,
                n,
                2.0 * n,
                2.0 * n,
                2.0 * n,
                3.0 * n,
                3.0 * n,
                3.0 * n,
                3.0 * n,
                4.0 * n,
                4.0 * n,
            ])
        }

        let vertices_byte_slice: &[u8] = bytemuck::cast_slice(vertices_vec.as_slice());
        let control_byte_slice: &[u8] = bytemuck::cast_slice(control_vec.as_slice());

        assert_eq!(vertices_byte_slice, control_byte_slice);
    }
}
