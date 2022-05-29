use crate::math::*;
use crate::renderer::asset::model::*;

pub fn cube_mesh(colour: Vec4f) -> Mesh {
    let vertices = vec![
        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
    ];

    Mesh {
        vertices,
        indices: vec![
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
        ],
    }
}

// fn cube(&self) -> VertexArray<'a> {
//     let layout = BufferLayout::new(vec![
//         BufferElement::new(shader::ShaderDataType::Float3, "vertices"),
//         BufferElement::new(shader::ShaderDataType::Float3, "normals"),
//         BufferElement::new(shader::ShaderDataType::Float2, "texCoords"),
//     ]);
//     #[cfg_attr(rustfmt, rustfmt_skip)]
//     let data: &[f32] = &[
//         0.5, -0.5, -0.5,    0.0, 1.0, 0.0,    0.0, 0.0,
//         0.5, -0.5, 0.5,     0.0, 1.0, 0.0,    0.0, 1.0,
//         -0.5, -0.5, 0.5,    0.0, 1.0, 0.0,    1.0, 1.0,
//         -0.5, -0.5, -0.5,   0.0, 1.0, 0.0,    1.0, 0.0,

//         -0.5, -0.5, -0.5,   0.0, 0.0, -1.0,    0.0, 0.0,
//         -0.5, 0.5, -0.5,    0.0, 0.0, -1.0,    0.0, 1.0,
//         0.5, 0.5, -0.5,     0.0, 0.0, -1.0,    1.0, 1.0,
//         0.5, -0.5, -0.5,    0.0, 0.0, -1.0,    1.0, 0.0,

//         -0.5, 0.5, -0.5,    0.0, 1.0, 0.0,     0.0, 0.0,
//         -0.5, 0.5, 0.5,     0.0, 1.0, 0.0,     0.0, 1.0,
//         0.5, 0.5, 0.5,      0.0, 1.0, 0.0,     1.0, 1.0,
//         0.5, 0.5, -0.5,     0.0, 1.0, 0.0,     1.0, 0.0,

//         0.5, -0.5, 0.5,     0.0, 0.0, 1.0,     0.0, 0.0,
//         0.5, 0.5, 0.5,      0.0, 0.0, 1.0,     0.0, 1.0,
//         -0.5, 0.5, 0.5,     0.0, 0.0, 1.0,     1.0, 1.0,
//         -0.5, -0.5, 0.5,    0.0, 0.0, 1.0,     1.0, 0.0,

//         0.5, -0.5, -0.5,    1.0, 0.0, 0.0,     0.0, 0.0,
//         0.5, 0.5, -0.5,     1.0, 0.0, 0.0,     0.0, 1.0,
//         0.5, 0.5, 0.5,      1.0, 0.0, 0.0,     1.0, 1.0,
//         0.5, -0.5, 0.5,     1.0, 0.0, 0.0,     1.0, 0.0,

//         -0.5, -0.5, 0.5,    -1.0, 0.0, 0.0,    0.0, 0.0,
//         -0.5, 0.5, 0.5,     -1.0, 0.0, 0.0,    0.0, 1.0,
//         -0.5, 0.5, -0.5,    -1.0, 0.0, 0.0,    1.0, 1.0,
//         -0.5, -0.5, -0.5,   -1.0, 0.0, 0.0,    1.0, 0.0,
//     ];

//     let buffer = VertexBuffer::new(self.gl, data, layout);

//     let indices: &[u32] = &[
//         0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
//         17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
//     ];
//     let index_buffer = IndexBuffer::new(self.gl, indices);

//     let vao = VertexArray::new(self.gl, buffer, index_buffer);
//     vao
// }
