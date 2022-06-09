use crate::{math::math::*, resource_manager::model::*};



pub fn axis() -> Mesh {
    let vertices = vec![
        Vertex {
            position: Vec3f::new(-0.2, 0.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, 0.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },

        Vertex {
            position: Vec3f::new(0.2, 0.02, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, -0.02, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, 0.0, 0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.2, 0.0, -0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(1.0, 0.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },


        Vertex {
            position: Vec3f::new(0.0, -0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },

        Vertex {
            position: Vec3f::new(0.02, 0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.02, 0.2, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.2, 0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.2, -0.02),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 1.0, 0.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },


        Vertex {
            position: Vec3f::new(0.0, 0.0, -0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.0, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        
        Vertex {
            position: Vec3f::new(0.02, 0.0, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.02, 0.0, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, 0.02, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour: Vec4f::new(0.0, 0.0, 1.0, 1.0),
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.0, -0.02, 0.2),
            normal: Vec3f::new(0.0, 0.0, 0.0),
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
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-1.0, -1.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-1.0, 1.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(1.0, 1.0, 0.0),
            normal: Vec3f::new(0.0, 0.0, 0.0),
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
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, -1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },


        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 0.0, -1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },


        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 1.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },


        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(0.0, 0.0, 1.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        

        Vertex {
            position: Vec3f::new(0.5, -0.5, -0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, -0.5, 0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, 0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(0.5, 0.5, -0.5),
            normal: Vec3f::new(1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
        
        
        Vertex {
            position: Vec3f::new(-0.5, -0.5, 0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, -0.5, -0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 0.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, -0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3f::new(-0.5, 0.5, 0.5),
            normal: Vec3f::new(-1.0, 0.0, 0.0),
            colour,
            tex_coord: Vec2f::new(0.0, 1.0),
        },
    ];

    Mesh {
        vertices,
        indices: vec![
            0, 1, 2, 2, 3, 0, 
            4, 5, 6, 6, 7, 4, 
            8, 9, 10, 10, 11, 8, 
            12, 13, 14, 14, 15, 12, 
            16, 17, 18, 18, 19, 16, 
            20, 21, 22, 22, 23, 20,
        ],
    }
}
