use super::texture::{TextureFilter, TextureWrap, TextureType};
use crate::{
    graphics::{
        framebuffer::*,
        graphics::ApiEnum,
        texture::{Texture, TextureConfig},
    },
    platform::rustgl as gl,
};

#[derive(Clone, Copy)]
pub enum InternalFormat {
    Depth16 = gl::DEPTH_COMPONENT16 as isize,
    Depth24 = gl::DEPTH_COMPONENT24 as isize,
    Depth32 = gl::DEPTH_COMPONENT32 as isize,
    Depth32F = gl::DEPTH_COMPONENT32F as isize,
    Depth24Stencil8 = gl::DEPTH24_STENCIL8 as isize,
    Depth32FStencil8 = gl::DEPTH32F_STENCIL8 as isize,
    Stencil8 = gl::STENCIL_INDEX8 as isize,
    Stencil16 = gl::STENCIL_INDEX16 as isize,
    RGBA8 = gl::RGBA8 as isize,
    RGBA16 = gl::RGBA16 as isize,
    RGBA16F = gl::RGBA16F as isize,
    RGBA32F = gl::RGBA32F as isize,
    RGB8 = gl::RGB8 as isize,
    RGB16 = gl::RGB16 as isize,
    RGB16F = gl::RGB16F as isize,
    RGB32F = gl::RGB32F as isize,
    RG8 = gl::RG8 as isize,
    RG16 = gl::RG16 as isize,
    RG16F = gl::RG16F as isize,
    RG32F = gl::RG32F as isize,
    R8 = gl::R8 as isize,
    R16 = gl::R16 as isize,
    R16F = gl::R16F as isize,
    R32F = gl::R32F as isize,
    SRGB8 = gl::SRGB8 as isize,
    SRGB8A8 = gl::SRGB8_ALPHA8 as isize,
    Bc6hUnsigned16F = gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT as isize,
    Bc6hSigned16F = gl::COMPRESSED_RGB_BPTC_SIGNED_FLOAT as isize,
    Bc7UnsignedNormalised = gl::COMPRESSED_RGBA_BPTC_UNORM as isize,
    Bc7UnsignedNormalisedSRGB = gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM as isize,
}

impl Framebuffer {
    pub fn new(config: &FramebufferConfig) -> Framebuffer {
        if config.samples > 1 {
            Self::new_multisample(config)
        } else {
            Self::new_monosample(config)
        }
    }

    pub fn new_monosample(config: &FramebufferConfig) -> Framebuffer {
        let config = config.clone();

        let framebuffer = unsafe { gl::create_named_framebuffer().unwrap() };

        let colour_handle = match config.colour {
            FramebufferAttachmentConfig::Renderbuffer { internal_format } => {
                Self::create_renderbuffer(
                    framebuffer,
                    gl::COLOR_ATTACHMENT0,
                    internal_format,
                    config.width,
                    config.height,
                )
            }
            FramebufferAttachmentConfig::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => Self::create_texture(
                target,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
                1,
            ),
            FramebufferAttachmentConfig::None => unsafe {
                gl::named_framebuffer_draw_buffer(framebuffer, gl::NONE);
                gl::named_framebuffer_read_buffer(framebuffer, gl::NONE);

                FramebufferAttachment::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachmentConfig::Renderbuffer { internal_format } => {
                Self::create_renderbuffer(
                    framebuffer,
                    gl::DEPTH_ATTACHMENT,
                    internal_format,
                    config.width,
                    config.height,
                )
            }
            FramebufferAttachmentConfig::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => Self::create_texture(
                target,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
                1,
            ),
            FramebufferAttachmentConfig::None => FramebufferAttachment::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachmentConfig::Renderbuffer { internal_format } => {
                Self::create_renderbuffer(
                    framebuffer,
                    gl::STENCIL_ATTACHMENT,
                    internal_format,
                    config.width,
                    config.height,
                )
            }
            FramebufferAttachmentConfig::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => Self::create_texture(
                target,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
                1,
            ),
            FramebufferAttachmentConfig::None => FramebufferAttachment::None,
        };

        Framebuffer {
            handle: framebuffer.0,
            colour_handle,
            depth_handle,
            stencil_handle,
            config,
        }
    }

    pub fn new_multisample(config: &FramebufferConfig) -> Framebuffer {
        let config = config.clone();

        let framebuffer = unsafe { gl::create_named_framebuffer().unwrap() };

        let colour_handle = match config.colour {
            FramebufferAttachmentConfig::Renderbuffer { internal_format } => {
                Self::create_renderbuffer_multisample(
                    framebuffer,
                    gl::COLOR_ATTACHMENT0,
                    internal_format,
                    config.width,
                    config.height,
                    config.samples,
                )
            }
            FramebufferAttachmentConfig::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => Self::create_texture(
                target,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
                config.samples,
            ),
            FramebufferAttachmentConfig::None => unsafe {
                gl::named_framebuffer_draw_buffer(framebuffer, gl::NONE);
                gl::named_framebuffer_read_buffer(framebuffer, gl::NONE);

                FramebufferAttachment::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachmentConfig::Renderbuffer { internal_format } => {
                Self::create_renderbuffer_multisample(
                    framebuffer,
                    gl::DEPTH_ATTACHMENT,
                    internal_format,
                    config.width,
                    config.height,
                    config.samples,
                )
            }
            FramebufferAttachmentConfig::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => Self::create_texture(
                target,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
                config.samples,
            ),
            FramebufferAttachmentConfig::None => FramebufferAttachment::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachmentConfig::Renderbuffer { internal_format } => {
                Self::create_renderbuffer_multisample(
                    framebuffer,
                    gl::STENCIL_ATTACHMENT,
                    internal_format,
                    config.width,
                    config.height,
                    config.samples,
                )
            }
            FramebufferAttachmentConfig::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => Self::create_texture(
                target,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
                config.samples,
            ),
            FramebufferAttachmentConfig::None => FramebufferAttachment::None,
        };

        Framebuffer {
            handle: framebuffer.0,
            colour_handle,
            depth_handle,
            stencil_handle,
            config,
        }
    }

    fn create_renderbuffer(
        framebuffer: gl::GlFramebuffer,
        attachment_type: ApiEnum,
        internal_format: InternalFormat,
        width: u32,
        height: u32,
    ) -> FramebufferAttachment {
        let rbo = unsafe { gl::create_named_renderbuffer().unwrap() };

        unsafe {
            gl::named_renderbuffer_storage(
                rbo,
                internal_format as u32,
                width as i32,
                height as i32,
            );
            gl::named_framebuffer_renderbuffer(framebuffer, attachment_type, gl::RENDERBUFFER, rbo);
        }

        FramebufferAttachment::Renderbuffer(rbo.0)
    }

    fn create_renderbuffer_multisample(
        framebuffer: gl::GlFramebuffer,
        attachment_type: ApiEnum,
        internal_format: InternalFormat,
        width: u32,
        height: u32,
        samples: u32,
    ) -> FramebufferAttachment {
        let rbo = unsafe { gl::create_named_renderbuffer().unwrap() };

        unsafe {
            gl::named_renderbuffer_storage_multisample(
                rbo,
                samples as i32,
                internal_format as u32,
                width as i32,
                height as i32,
            );
            gl::named_framebuffer_renderbuffer(framebuffer, attachment_type, gl::RENDERBUFFER, rbo);
        }

        FramebufferAttachment::Renderbuffer(rbo.0)
    }

    fn create_texture(
        target: &TextureType,
        framebuffer: gl::GlFramebuffer,
        attachment_type: ApiEnum,
        internal_format: InternalFormat,
        width: u32,
        height: u32,
        layers: u32,
        levels: u32,
        samples: u32,
    ) -> FramebufferAttachment {
        let texture = Texture::new_framebuffer_texture(
            *target,
            internal_format,
            layers,
            levels,
            samples,
            width,
            height,
            &TextureConfig {
                wrap: TextureWrap::ClampToEdge,
                min_filter: TextureFilter::Linear,
                mag_filter: TextureFilter::Linear,
                mipmap: levels > 1,
                srgb: false,
            },
        );

        unsafe {
            gl::named_framebuffer_texture(
                framebuffer,
                attachment_type,
                gl::GlTexture(texture.handle),
                0,
            )
        };

        FramebufferAttachment::Texture(texture)
    }

    pub fn blit_to_default_framebuffer(
        &self,
        src_x0: u32,
        src_y0: u32,
        src_x1: u32,
        src_y1: u32,
        dst_x0: u32,
        dst_y0: u32,
        dst_x1: u32,
        dst_y1: u32,
        filter: TextureFilter,
    ) {
        unsafe {
            gl::blit_to_default_framebuffer(
                gl::GlFramebuffer(self.handle),
                src_x0 as i32,
                src_y0 as i32,
                src_x1 as i32,
                src_y1 as i32,
                dst_x0 as i32,
                dst_y0 as i32,
                dst_x1 as i32,
                dst_y1 as i32,
                gl::COLOR_BUFFER_BIT,
                filter as u32,
            )
        }
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::clear_named_framebuffer_f32_slice(
                gl::GlFramebuffer(self.handle),
                gl::COLOR,
                0,
                &[r, g, b, a],
            )
        }
    }

    pub fn clear_depth(&self, value: f32) {
        unsafe {
            gl::clear_named_framebuffer_f32(gl::GlFramebuffer(self.handle), gl::DEPTH, 0, value)
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        let new = Self::new(&self.config);
        self.delete();
        *self = new;
    }

    pub fn bind(&self) {
        unsafe {
            gl::bind_framebuffer(gl::FRAMEBUFFER, Some(gl::GlFramebuffer(self.handle)));
            gl::viewport(0, 0, self.config.width as i32, self.config.height as i32);
        }
    }

    pub fn unbind() {
        unsafe { gl::bind_framebuffer(gl::FRAMEBUFFER, None) }
    }

    pub fn delete(&self) {
        Self::unbind();

        match self.colour_handle {
            FramebufferAttachment::Renderbuffer(handle) => unsafe {
                gl::delete_renderbuffer(gl::GlRenderbuffer(handle))
            },
            FramebufferAttachment::Texture(ref texture) => unsafe {
                gl::delete_texture(gl::GlTexture(texture.handle))
            },
            FramebufferAttachment::None => (),
        }

        match self.depth_handle {
            FramebufferAttachment::Renderbuffer(handle) => unsafe {
                gl::delete_renderbuffer(gl::GlRenderbuffer(handle))
            },
            FramebufferAttachment::Texture(ref texture) => unsafe {
                gl::delete_texture(gl::GlTexture(texture.handle))
            },
            FramebufferAttachment::None => (),
        }

        match self.stencil_handle {
            FramebufferAttachment::Renderbuffer(handle) => unsafe {
                gl::delete_renderbuffer(gl::GlRenderbuffer(handle))
            },
            FramebufferAttachment::Texture(ref texture) => unsafe {
                gl::delete_texture(gl::GlTexture(texture.handle))
            },
            FramebufferAttachment::None => (),
        }

        unsafe { gl::delete_framebuffer(gl::GlFramebuffer(self.handle)) }
    }
}
