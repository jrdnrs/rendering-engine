#[cfg(feature = "opengl")]
pub use super::opengl::framebuffer::InternalFormat;
use super::{texture::{TextureType, Texture}, ApiHandle};

#[derive(Clone)]
pub enum FramebufferAttachmentConfig {
    Renderbuffer {
        internal_format: InternalFormat,
    },
    Texture {
        target: TextureType,
        internal_format: InternalFormat,
        layers: u32,
        levels: u32,
    },
    None,
}

pub enum FramebufferAttachment {
    Renderbuffer(ApiHandle),
    Texture(Texture),
    None,
}


#[derive(Clone)]
pub struct FramebufferConfig {
    pub colour: FramebufferAttachmentConfig,
    pub depth: FramebufferAttachmentConfig,
    pub stencil: FramebufferAttachmentConfig,

    pub width: u32,
    pub height: u32,
    pub samples: u32,
}

impl Default for FramebufferConfig {
    fn default() -> Self {
        Self {
            colour: FramebufferAttachmentConfig::None,
            depth: FramebufferAttachmentConfig::None,
            stencil: FramebufferAttachmentConfig::None,
            width: crate::WIDTH as u32,
            height: crate::HEIGHT as u32,
            samples: 1,
        }
    }
}

pub struct Framebuffer {
    pub handle: ApiHandle,
    pub colour_handle: FramebufferAttachment,
    pub depth_handle: FramebufferAttachment,
    pub stencil_handle: FramebufferAttachment,

    pub config: FramebufferConfig,
}
