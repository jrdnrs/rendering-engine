#[cfg(feature = "opengl")]
pub use super::opengl::texture::{ImageFormat, TextureFilter, TextureWrap, TextureType};
use super::{opengl::framebuffer::InternalFormat, ApiEnum, ApiHandle, DataType};

#[derive(Clone)]
pub struct TextureConfig {
    pub wrap: TextureWrap,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub mipmap: bool,
    pub srgb: bool,
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            wrap: TextureWrap::Repeat,
            min_filter: TextureFilter::Linear,
            mag_filter: TextureFilter::Linear,
            mipmap: false,
            srgb: false,
        }
    }
}

pub struct Texture {
    pub config: TextureConfig,
    pub handle: ApiHandle,
    pub shader_texture_handle: Option<u64>,
    pub resident: bool,
    pub target: ApiEnum,
    pub internal_format: InternalFormat,
    pub format: ImageFormat,
    pub data_type: DataType,
    pub width: u32,
    pub height: u32,
}

pub struct Image {
    pub path: &'static str,
    pub format: ImageFormat,
    pub data_type: DataType,
    pub compressed: bool,
    pub mipmap_count: u32,
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            path: Default::default(),
            format: ImageFormat::RGBA,
            data_type: DataType::Float32,
            compressed: Default::default(),
            mipmap_count: Default::default(),
            width: Default::default(),
            height: Default::default(),
            bytes: Default::default(),
        }
    }
}
