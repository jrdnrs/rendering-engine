use crate::math::math::*;

type Padding = u32;

#[repr(C)]
pub struct ShaderStorageBuffers {
    pub lights: LightsStorageBuffer,
    pub skybox: SkyboxStorageBuffer,
    pub materials: MaterialsStorageBuffer,
    pub matrices: MatricesStorageBuffer,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LightsStorageBuffer {
    pub all_lights: [Light; 32],
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
#[derive(Default, Clone, Copy)]
pub struct SkyboxStorageBuffer {
    pub skybox_texture: Vec2u,
    pub _1: Padding,
    pub _2: Padding,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Light {
    pub ambient_col: Vec3f,
    pub ambient_strength: f32,

    pub diffuse_col: Vec3f,
    pub diffuse_strength: f32,

    pub specular_col: Vec3f,
    pub specular_strength: f32,

    pub inner_cutoff: f32,
    pub outer_cutoff: f32,

    pub quadratic: f32,
    pub linear: f32,
    pub constant: f32,
    pub _1: Padding,
    pub _2: Padding,
    pub _3: Padding,

    pub position: Vec3f,
    pub _4: Padding,
    pub direction: Vec3f,
    pub _5: Padding,
 
}

unsafe impl bytemuck::Pod for Light {}
unsafe impl bytemuck::Zeroable for Light {}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct MaterialsStorageBuffer {
    pub materials: [Material; 100],
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Material {
    pub shininess: f32,
    pub _3: Padding,
    pub diffuse_texture: Vec2u,

}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MatricesStorageBuffer {
    pub projection: Mat4f,
    pub view: Mat4f,
}
