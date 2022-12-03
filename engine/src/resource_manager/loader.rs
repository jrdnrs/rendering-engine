use tobj;

use crate::math::Vec3f;

const LOAD_OPTIONS: tobj::LoadOptions = tobj::LoadOptions {
    single_index: true,
    triangulate: true,
    ignore_points: false,
    ignore_lines: false,
};

pub struct Material {
    pub name: String,
    pub ambient_col: Vec3f,
    pub diffuse_col: Vec3f,
    pub specular_col: Vec3f,
    pub shininess: f32,
    pub diffuse_texture: String,
    pub specular_texture: String,
}

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub material_id: Option<usize>,
}

impl Mesh {
    pub fn new(mesh: tobj::Mesh) -> Self {
        let mut m = Mesh {
            vertices: Vec::new(),
            indices: mesh.indices,
            material_id: mesh.material_id,
        };

        for ((pos, normal), tex_coord) in mesh
            .positions
            .chunks_exact(3)
            .zip(mesh.normals.chunks_exact(3))
            .zip(mesh.texcoords.chunks_exact(2))
        {
            m.vertices.extend_from_slice(pos);
            m.vertices.extend_from_slice(normal);
            // calc tangent
            m.vertices.extend_from_slice(&[1.0, 1.0, 1.0, 1.0]); // colour
            m.vertices.extend_from_slice(tex_coord);
        }

        m
    }
}

pub struct Model {
    pub filepath: &'static str,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn new(filepath: &'static str) -> Result<Self, String> {
        let mut m = Model {
            filepath,
            meshes: Vec::new(),
            materials: Vec::new(),
        };

        if let Ok((models, material_result)) = tobj::load_obj(m.filepath, &LOAD_OPTIONS) {
            for model in models {
                m.meshes.push(Mesh::new(model.mesh));
            }

            if let Ok(materials) = material_result {
                for material in materials {
                    m.materials.push(Material {
                        name: material.name,
                        ambient_col: Vec3f::from_array(material.ambient),
                        diffuse_col: Vec3f::from_array(material.diffuse),
                        specular_col: Vec3f::from_array(material.specular),
                        shininess: material.shininess,
                        diffuse_texture: material.diffuse_texture,
                        specular_texture: material.specular_texture,
                    })
                }
            }

            return Ok(m);
        } else {
            return Err("Failed to load object file".to_string());
        }
    }
}
