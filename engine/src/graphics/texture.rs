#[cfg(feature = "opengl")]
pub use super::opengl::texture::{ImageDataType, ImageFormat, TextureFilter, TextureWrap};
use super::{ApiEnum, ApiHandle};

#[derive(Clone, Copy)]
pub enum TextureType {
    T2D,
    T2DArray,
    T3D,
    CubeMap,
}

// #[derive(Clone, Copy)]
// pub enum TextureFilter {
//     Nearest,
//     Linear,
// }

// #[derive(Clone, Copy)]
// pub enum TextureWrap {
//     ClampToEdge,
//     ClampToBorder,
//     Repeat,
//     MirrorRepeat,
//     MirrorClampToEdge,
// }

// #[derive(Clone, Copy)]
// pub enum ImageDataType {
//     UnsignedByte,
//     UnsignedShort,
//     Float,
//     NA,
// }

// #[derive(Clone, Copy)]
// pub enum ImageFormat {
//     RGB,
//     RGBA,
//     Bc6hTypeless,
//     Bc6hUnsignedFloat16,
//     Bc6hSignedFloat16,
//     Bc7Typeless,
//     Bc7UnsignedNormalised,
//     Bc7UnsignedNormalisedSrgb,
// }

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
    pub target: ApiEnum,
    pub internal_format: ApiEnum,
    pub format: ApiEnum,
    pub data_type: ApiEnum,
    pub width: usize,
    pub height: usize,
}

pub struct Image {
    pub path: &'static str,
    pub format: ImageFormat,
    pub data_type: ImageDataType,
    pub compressed: bool,
    pub mipmap_count: usize,
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<u8>,
}
