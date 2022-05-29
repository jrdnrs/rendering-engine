use crate::math::*;

type Padding = u32;

#[repr(C)]
#[derive(Default)]
pub struct MaterialUniform {
    pub ambient_col: Vec3f,
    pub _1: Padding,
    pub diffuse_col: Vec3f,
    pub _2: Padding,
    pub specular_col: Vec3f,
    pub shininess: f32,
    pub diffuse_texture: Vec2u,
    pub _3: Padding,
    pub _4: Padding,
}

#[repr(C)]
#[derive(Default)]
pub struct LightUniform {
    pub ambient_strength: f32,
    pub diffuse_strength: f32,
    pub specular_strength: f32,
    pub inner_cutoff: f32,
    pub outer_cutoff: f32,
    pub quadratic: f32,
    pub linear: f32,
    pub constant: f32,
    pub position: Vec3f,
    pub _1: Padding,
    pub direction: Vec3f,
    pub _2: Padding,
}

#[repr(C)]
pub struct VertexUniform {
    pub materials: [MaterialUniform; 100],
    pub transforms: [Mat4f; 1_000],
    pub projection: Mat4f,
    pub view: Mat4f,
    pub material_index: [u32; 1_000],
}

#[repr(C)]
pub struct FragmentUniform {
    pub all_lights: [LightUniform; 32],
    pub light_count: u32,
    pub _1: Padding,
    pub _2: Padding,
    pub _3: Padding,
    pub camera_dir: Vec3f,
    pub _4: Padding,
    pub camera_pos: Vec3f,
    pub _5: Padding,
}

#[repr(C)]
pub struct Uniforms {
    pub vertex: VertexUniform,
    pub fragment: FragmentUniform,
}

mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn alignment_size_test() {
        type Uniforms_std140 = Uniforms;
        type VertexUniform_std140 = VertexUniform;
        type FragmentUniform_std140 = FragmentUniform;

        let vert_uni_offset = memoffset::offset_of!(Uniforms_std140, vertex);
        let frag_uni_offset = memoffset::offset_of!(Uniforms_std140, fragment);

        println!("\nByte offsets:");
        println!("VertexUniform: {}", vert_uni_offset);
        println!(
            "materials: {}",
            memoffset::offset_of!(VertexUniform_std140, materials)
        );
        println!(
            "transforms: {}",
            memoffset::offset_of!(VertexUniform_std140, transforms)
        );
        println!(
            "projection: {}",
            memoffset::offset_of!(VertexUniform_std140, projection)
        );
        println!(
            "view: {}",
            memoffset::offset_of!(VertexUniform_std140, view)
        );

        println!("");

        println!("FragmentUniform: {} ({})", 0, frag_uni_offset);
        println!(
            "all_lights: {} ({})",
            memoffset::offset_of!(FragmentUniform_std140, all_lights),
            memoffset::offset_of!(FragmentUniform_std140, all_lights) + frag_uni_offset
        );
        println!(
            "light_count: {} ({})",
            memoffset::offset_of!(FragmentUniform_std140, light_count),
            memoffset::offset_of!(FragmentUniform_std140, light_count) + frag_uni_offset
        );
        println!(
            "camera_dir: {} ({})",
            memoffset::offset_of!(FragmentUniform_std140, camera_dir),
            memoffset::offset_of!(FragmentUniform_std140, camera_dir) + frag_uni_offset
        );
        println!(
            "camera_pos: {} ({})",
            memoffset::offset_of!(FragmentUniform_std140, camera_pos),
            memoffset::offset_of!(FragmentUniform_std140, camera_pos) + frag_uni_offset
        );

        println!("");

        println!("VertexUniform size: {}", size_of::<VertexUniform_std140>());
        println!(
            "FragmentUniform size: {}",
            size_of::<FragmentUniform_std140>()
        );
        println!("");
    }
}
