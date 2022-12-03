#[cfg(feature = "opengl")]
pub use super::opengl::framebuffer::InternalFormat;
use super::{texture::TextureType, ApiHandle};

#[derive(Clone)]
pub enum FramebufferAttachment {
    Renderbuffer {
        internal_format: InternalFormat,
    },
    Texture {
        target: TextureType,
        internal_format: InternalFormat,
        layers: usize,
        levels: usize,
    },
    None,
}

pub enum FramebufferAttachmentHandle {
    Renderbuffer(ApiHandle),
    Texture(ApiHandle),
    None,
}

// #[derive(Clone, Copy)]
// pub enum InternalFormat {
//     Depth16,
//     Depth24,
//     Depth32,
//     Depth32F,
//     Depth24Stencil8,
//     Depth32FStencil8,
//     Stencil8,
//     Stencil16,
//     RGBA8,
//     RGBA16,
//     RGBA16F,
//     RGBA32F,
//     RGB8,
//     RGB16,
//     RGB16F,
//     RGB32F,
//     RG8,
//     RG16,
//     RG16F,
//     RG32F,
//     R8,
//     R16,
//     R16F,
//     R32F,
// }

#[derive(Clone)]
pub struct FramebufferConfig {
    pub colour: FramebufferAttachment,
    pub depth: FramebufferAttachment,
    pub stencil: FramebufferAttachment,

    pub width: usize,
    pub height: usize,
    pub samples: usize,
}

impl Default for FramebufferConfig {
    fn default() -> Self {
        Self {
            colour: FramebufferAttachment::None,
            depth: FramebufferAttachment::None,
            stencil: FramebufferAttachment::None,
            width: crate::WIDTH as usize,
            height: crate::HEIGHT as usize,
            samples: 1,
        }
    }
}

pub struct Framebuffer {
    pub handle: ApiHandle,
    pub colour_handle: FramebufferAttachmentHandle,
    pub depth_handle: FramebufferAttachmentHandle,
    pub stencil_handle: FramebufferAttachmentHandle,

    pub config: FramebufferConfig,
}
