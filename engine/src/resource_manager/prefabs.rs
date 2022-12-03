use crate::{math::*, resource_manager::model::*};

pub fn axis() -> Mesh {
    let vertices = vec![
        Vertex {
            position: Vec3f::new(-0.2, 0.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, 0.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, 0.02, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, -0.02, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, 0.0, 0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, 0.0, -0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, -0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.02, 0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.02, 0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.2, 0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.2, -0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.0, -0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.0, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.02, 0.0, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.02, 0.0, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.02, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, -0.02, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
    ];

    Mesh {
        vertices,
        indices: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    }
}

pub fn quad_mesh(colour: Vec4f) -> Mesh {
    let vertices = vec![
        Vertex {
            position: Vec3f::new(1.0, -1.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-1.0, -1.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-1.0, 1.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(1.0, 1.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
    ];

    Mesh {
        vertices,
        indices: vec![0, 1, 2, 2, 3, 0],
    }
}

pub fn unit_cube_mesh(colour: Vec4f) -> Mesh {
    let vertices = vec![
        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            tangent: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            tangent: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            tangent: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
    ];

    Mesh {
        vertices,
        indices: vec![
            0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16,
            17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
        ],
    }
}

pub fn sphere(resolution: u32) -> Mesh {
    let origins = [
        Vec3f::new(-1.0, -1.0, -1.0),
        Vec3f::new(1.0, -1.0, -1.0),
        Vec3f::new(1.0, -1.0, 1.0),
        Vec3f::new(-1.0, -1.0, 1.0),
        Vec3f::new(-1.0, 1.0, -1.0),
        Vec3f::new(-1.0, -1.0, 1.0),
    ];

    let rights = [
        Vec3f::new(1.0, 0.0, 0.0),
        Vec3f::new(0.0, 0.0, 1.0),
        Vec3f::new(-1.0, 0.0, 0.0),
        Vec3f::new(0.0, 0.0, -1.0),
        Vec3f::new(1.0, 0.0, 0.0),
        Vec3f::new(1.0, 0.0, 0.0),
    ];

    let ups = [
        Vec3f::new(0.0, 1.0, 0.0),
        Vec3f::new(0.0, 1.0, 0.0),
        Vec3f::new(0.0, 1.0, 0.0),
        Vec3f::new(0.0, 1.0, 0.0),
        Vec3f::new(0.0, 0.0, 1.0),
        Vec3f::new(0.0, 0.0, -1.0),
    ];

    // let forwards = [
    //     Vec3f::new(0.0, 0.0, 1.0),
    //     Vec3f::new(-1.0, 0.0, 0.0),
    //     Vec3f::new(0.0, 0.0, -1.0),
    //     Vec3f::new(1.0, 0.0, 0.0),
    //     Vec3f::new(0.0, -1.0, 0.0),
    //     Vec3f::new(0.0, 1.0, 0.0),
    // ];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let step = 2.0 / resolution as f32;

    for face in 0..6 {
        let origin = origins[face];
        let right = rights[face];
        let up = ups[face];
        // let forward = forwards[face];

        for u in 0..resolution {
            for v in 0..resolution {
                indices.push(vertices.len() as u32 + v + u * (resolution + 1));
                indices.push(vertices.len() as u32 + v + (u + 1) * (resolution + 1));
                indices.push(vertices.len() as u32 + (v + 1) + (u + 1) * (resolution + 1));
                indices.push(vertices.len() as u32 + (v + 1) + (u + 1) * (resolution + 1));
                indices.push(vertices.len() as u32 + (v + 1) + u * (resolution + 1));
                indices.push(vertices.len() as u32 + v + u * (resolution + 1));
            }
        }

        for u in 0..=resolution {
            for v in 0..=resolution {
                let p = origin + (right * u as f32 + up * v as f32) * step;
                // let n = p.normalise();

                let x2 = p.x * p.x;
                let y2 = p.y * p.y;
                let z2 = p.z * p.z;

                let n = Vec3f::new(
                    p.x * (1.0 - (y2 + z2) / 2.0 + y2 * z2 / 3.0).sqrt(),
                    p.y * (1.0 - (x2 + z2) / 2.0 + x2 * z2 / 3.0).sqrt(),
                    p.z * (1.0 - (x2 + y2) / 2.0 + x2 * y2 / 3.0).sqrt(),
                );

                vertices.push(Vertex {
                    position: n,
                    normal: n,
                    tangent: p.cross(-up).normalise(),
                    colour: Vec4f::uniform(1.0),
                    tex_coord: Vec2f::new(u as f32, v as f32) / resolution as f32,
                });
            }
        }
    }

    Mesh { vertices, indices }
}
