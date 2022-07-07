use crate::math::*;

pub const MAX_POINT_LIGHTS: usize = 2;
pub const MAX_SPOT_LIGHTS: usize = 4;

type Padding = u32;

#[repr(C)]
/// Considered static because the data will expect to persist without modification for multiple frames at least
pub struct StaticShaderStorageBuffers {
    pub materials: MaterialsStorageBuffer,
    pub shadow_maps: ShadowMapStorageBuffer,
}

#[repr(C)]
/// Used as space for parameters that are expected to change each frame
pub struct FrameShaderStorageBuffers {
    pub lights: LightsStorageBuffer,
    pub matrices: MatricesStorageBuffer,
}

#[repr(C)]
/// Used as space for parameters that are expected to change each draw call
pub struct DrawShaderStorageBuffers {
    pub general: [GeneralPurposeStorageBuffer; 100],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GeneralPurposeStorageBuffer {
    pub indices: GeneralPurposeIndexStorageBuffer,
    pub vecs: GeneralPurposeVecStorageBuffer,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GeneralPurposeIndexStorageBuffer {
    pub index_1: u32,
    pub index_2: u32,
    pub index_3: u32,
    pub index_4: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GeneralPurposeVecStorageBuffer {
    pub vec_1: Vec4f,
    pub vec_2: Vec4f,
    pub vec_3: Vec4f,
    pub vec_4: Vec4f,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShadowMapStorageBuffer {
    pub point_shadow_map: [ShadowMap; MAX_POINT_LIGHTS],
    pub spot_shadow_map: [ShadowMap; MAX_SPOT_LIGHTS],
    pub directional_shadow_map: ShadowMap,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShadowMap {
    pub handle: Vec2u,
    pub _1: Padding,
    pub _2: Padding,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct LightsStorageBuffer {
    pub point_lights: [PointLight; MAX_POINT_LIGHTS],
    pub spot_lights: [SpotLight; MAX_SPOT_LIGHTS],
    pub directional_light: DirectionalLight,

    pub point_light_count: u32,
    pub spot_light_count: u32,
    pub directional_light_count: u32,
    pub _1: Padding,

    pub camera_dir: Vec3f,
    pub _2: Padding,
    pub camera_pos: Vec3f,
    pub _3: Padding,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct PointLight {
    pub ambient_col: Vec3f,
    pub _1: Padding,
    pub diffuse_col: Vec3f,
    pub _2: Padding,
    pub specular_col: Vec3f,
    pub _3: Padding,

    // quadratic, linear, constant
    pub attenuation: Vec3f,
    pub _4: Padding,

    pub position: Vec3f,
    pub _5: Padding,

    pub views: [Mat4f; 6],
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct SpotLight {
    pub ambient_col: Vec3f,
    pub _1: Padding,
    pub diffuse_col: Vec3f,
    pub _2: Padding,
    pub specular_col: Vec3f,
    pub _3: Padding,

    // quadratic, linear, constant
    pub attenuation: Vec3f,

    pub inner_cutoff: f32,
    pub outer_cutoff: f32,
    pub _4: Padding,
    pub _5: Padding,
    pub _6: Padding,

    pub position: Vec3f,
    pub _7: Padding,
    pub direction: Vec3f,
    pub _8: Padding,

    pub view: Mat4f,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct DirectionalLight {
    pub ambient_col: Vec3f,
    pub _1: Padding,
    pub diffuse_col: Vec3f,
    pub _2: Padding,
    pub specular_col: Vec3f,
    pub _3: Padding,

    pub position: Vec3f,
    pub _5: Padding,
    pub direction: Vec3f,
    pub _6: Padding,

    pub view: Mat4f,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct MaterialsStorageBuffer {
    pub materials: [Material; 100],
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Material {
    pub shininess: f32,
    pub _1: Padding,
    pub diffuse_texture: Vec2u,
    pub specular_texture: Vec2u,
    pub normal_texture: Vec2u,

}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MatricesStorageBuffer {
    pub projection: Mat4f,
    pub view: Mat4f,
}

unsafe impl bytemuck::Pod for PointLight {}
unsafe impl bytemuck::Zeroable for PointLight {}

unsafe impl bytemuck::Pod for SpotLight {}
unsafe impl bytemuck::Zeroable for SpotLight {}

unsafe impl bytemuck::Pod for DirectionalLight {}
unsafe impl bytemuck::Zeroable for DirectionalLight {}

