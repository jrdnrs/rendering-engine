use crate::{
    graphics::{framebuffer::*, graphics::ApiEnum, texture::TextureType},
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
}

fn get_gl_texture_type(t_type: &TextureType, multisample: bool) -> Result<u32, String> {
    if multisample {
        match t_type {
            TextureType::T2D => Ok(gl::TEXTURE_2D_MULTISAMPLE),
            TextureType::T2DArray => {
                Err("Cannot create multisampled array texture for framebuffer".to_string())
            }
            TextureType::T3D => {
                Err("Cannot create multisampled 3D texture for framebuffer".to_string())
            }
            TextureType::CubeMap => {
                Err("Cannot create multisampled Cubemap texture for framebuffer".to_string())
            }
        }
    } else {
        match t_type {
            TextureType::T2D => Ok(gl::TEXTURE_2D),
            TextureType::T2DArray => Ok(gl::TEXTURE_2D_ARRAY),
            TextureType::T3D => Ok(gl::TEXTURE_3D),
            TextureType::CubeMap => Ok(gl::TEXTURE_CUBE_MAP),
        }
    }
}

impl Framebuffer {
    pub fn new_monosample(&self, config: &FramebufferConfig) -> Framebuffer {
        let config = config.clone();

        let framebuffer = unsafe { gl::create_named_framebuffer().unwrap() };

        let colour_handle = match config.colour {
            FramebufferAttachment::Renderbuffer { internal_format } => self.create_renderbuffer(
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => self.create_texture(
                target,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
            ),
            FramebufferAttachment::None => unsafe {
                gl::named_framebuffer_draw_buffer(framebuffer, gl::NONE);
                gl::named_framebuffer_read_buffer(framebuffer, gl::NONE);

                FramebufferAttachmentHandle::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachment::Renderbuffer { internal_format } => self.create_renderbuffer(
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => self.create_texture(
                target,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachment::Renderbuffer { internal_format } => self.create_renderbuffer(
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => self.create_texture(
                target,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                layers,
                levels,
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        Framebuffer {
            handle: framebuffer.0,
            colour_handle,
            depth_handle,
            stencil_handle,
            config,
        }
    }

    pub fn new_multisample(&self, config: &FramebufferConfig) -> Framebuffer {
        let config = config.clone();

        let framebuffer = unsafe { gl::create_named_framebuffer().unwrap() };

        let colour_handle = match config.colour {
            FramebufferAttachment::Renderbuffer { internal_format } => self
                .create_renderbuffer_multisample(
                    framebuffer,
                    gl::COLOR_ATTACHMENT0,
                    internal_format,
                    config.width,
                    config.height,
                    config.samples,
                ),
            FramebufferAttachment::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => self.create_texture_multisample(
                target,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
                config.samples,
            ),
            FramebufferAttachment::None => unsafe {
                gl::named_framebuffer_draw_buffer(framebuffer, gl::NONE);
                gl::named_framebuffer_read_buffer(framebuffer, gl::NONE);

                FramebufferAttachmentHandle::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachment::Renderbuffer { internal_format } => self
                .create_renderbuffer_multisample(
                    framebuffer,
                    gl::DEPTH_ATTACHMENT,
                    internal_format,
                    config.width,
                    config.height,
                    config.samples,
                ),
            FramebufferAttachment::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => self.create_texture_multisample(
                target,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                config.samples,
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachment::Renderbuffer { internal_format } => self
                .create_renderbuffer_multisample(
                    framebuffer,
                    gl::STENCIL_ATTACHMENT,
                    internal_format,
                    config.width,
                    config.height,
                    config.samples,
                ),
            FramebufferAttachment::Texture {
                ref target,
                internal_format,
                layers,
                levels,
            } => self.create_texture_multisample(
                target,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                config.samples,
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
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
        &self,
        framebuffer: gl::GlFramebuffer,
        attachment_type: ApiEnum,
        internal_format: InternalFormat,
        width: usize,
        height: usize,
    ) -> FramebufferAttachmentHandle {
        let rbo = unsafe { gl::create_named_renderbuffer().unwrap() };

        unsafe {
            gl::named_renderbuffer_storage(rbo, internal_format as u32, width as i32, height as i32);
            gl::named_framebuffer_renderbuffer(framebuffer, attachment_type, gl::RENDERBUFFER, rbo);
        }

        FramebufferAttachmentHandle::Renderbuffer(rbo.0)
    }

    fn create_renderbuffer_multisample(
        &self,
        framebuffer: gl::GlFramebuffer,
        attachment_type: ApiEnum,
        internal_format: InternalFormat,
        width: usize,
        height: usize,
        samples: usize,
    ) -> FramebufferAttachmentHandle {
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

        FramebufferAttachmentHandle::Renderbuffer(rbo.0)
    }

    fn create_texture(
        &self,
        target: &TextureType,
        framebuffer: gl::GlFramebuffer,
        attachment_type: ApiEnum,
        internal_format: InternalFormat,
        width: usize,
        height: usize,
        depth: usize,
        levels: usize,
    ) -> FramebufferAttachmentHandle {
        let texture = unsafe {
            gl::create_named_texture(get_gl_texture_type(target, false).unwrap()).unwrap()
        };

        unsafe {
            match target {
                TextureType::T2DArray => {
                    gl::texture_storage_3d(
                        texture,
                        levels as i32,
                        internal_format as u32,
                        width as i32,
                        height as i32,
                        depth as i32,
                    );
                }
                _ => {
                    gl::texture_storage_2d(
                        texture,
                        levels as i32,
                        internal_format as u32,
                        width as i32,
                        height as i32,
                    );
                }
            }

            if levels > 1 {
                gl::texture_parameter_i32(
                    texture,
                    gl::TEXTURE_MIN_FILTER,
                    gl::LINEAR_MIPMAP_LINEAR as i32,
                );
            } else {
                gl::texture_parameter_i32(texture, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            }
            gl::texture_parameter_i32(texture, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::texture_parameter_i32(texture, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::texture_parameter_i32(texture, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::texture_parameter_i32(texture, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
            gl::texture_parameter_i32(
                texture,
                gl::TEXTURE_COMPARE_MODE,
                gl::COMPARE_REF_TO_TEXTURE as i32,
            );
            gl::texture_parameter_i32(texture, gl::TEXTURE_COMPARE_FUNC, gl::GREATER as i32);
            gl::named_framebuffer_texture(framebuffer, attachment_type, texture, 0);
        }

        FramebufferAttachmentHandle::Texture(texture.0)
    }

    fn create_texture_multisample(
        &self,
        target: &TextureType,
        framebuffer: gl::GlFramebuffer,
        attachment_type: ApiEnum,
        internal_format: InternalFormat,
        width: usize,
        height: usize,
        samples: usize,
    ) -> FramebufferAttachmentHandle {
        let texture = unsafe {
            gl::create_named_texture(get_gl_texture_type(target, true).unwrap()).unwrap()
        };

        unsafe {
            gl::texture_storage_2d_multisample(
                texture,
                samples as i32,
                internal_format as u32,
                width as i32,
                height as i32,
                true,
            );
            gl::named_framebuffer_texture(framebuffer, attachment_type, texture, 0);
        }

        FramebufferAttachmentHandle::Texture(texture.0)
    }

    pub fn get_texture_handle(handle: &FramebufferAttachmentHandle) -> Option<gl::GlTexture> {
        if let FramebufferAttachmentHandle::Texture(gl_handle) = handle {
            Some(gl::GlTexture(*gl_handle))
        } else {
            None
        }
    }

    pub fn get_renderbuffer_handle(
        handle: &FramebufferAttachmentHandle,
    ) -> Option<gl::GlRenderbuffer> {
        if let FramebufferAttachmentHandle::Texture(gl_handle) = handle {
            Some(gl::GlRenderbuffer(*gl_handle))
        } else {
            None
        }
    }

    pub fn resize(&self, framebuffer: &mut Framebuffer, width: usize, height: usize) {
        framebuffer.config.width = width;
        framebuffer.config.height = height;
        let new = if framebuffer.config.samples > 1 {
            self.new_multisample(&framebuffer.config)
        } else {
            self.new_monosample(&framebuffer.config)
        };
        self.delete(&framebuffer);
        *framebuffer = new;
    }

    pub fn bind(&self, framebuffer: &Framebuffer) {
        unsafe {
            gl::bind_framebuffer(gl::FRAMEBUFFER, Some(gl::GlFramebuffer(framebuffer.handle)));
            gl::viewport(
                0,
                0,
                framebuffer.config.width as i32,
                framebuffer.config.height as i32,
            );
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::bind_framebuffer(gl::FRAMEBUFFER, None) }
    }

    pub fn delete(&self, framebuffer: &Framebuffer) {
        self.unbind();

        match framebuffer.colour_handle {
            FramebufferAttachmentHandle::Renderbuffer(handle) => unsafe {
                gl::delete_renderbuffer(gl::GlRenderbuffer(handle))
            },
            FramebufferAttachmentHandle::Texture(handle) => unsafe {
                gl::delete_texture(gl::GlTexture(handle))
            },
            FramebufferAttachmentHandle::None => (),
        }

        match framebuffer.depth_handle {
            FramebufferAttachmentHandle::Renderbuffer(handle) => unsafe {
                gl::delete_renderbuffer(gl::GlRenderbuffer(handle))
            },
            FramebufferAttachmentHandle::Texture(handle) => unsafe {
                gl::delete_texture(gl::GlTexture(handle))
            },
            FramebufferAttachmentHandle::None => (),
        }

        match framebuffer.stencil_handle {
            FramebufferAttachmentHandle::Renderbuffer(handle) => unsafe {
                gl::delete_renderbuffer(gl::GlRenderbuffer(handle))
            },
            FramebufferAttachmentHandle::Texture(handle) => unsafe {
                gl::delete_texture(gl::GlTexture(handle))
            },
            FramebufferAttachmentHandle::None => (),
        }

        unsafe { gl::delete_framebuffer(gl::GlFramebuffer(framebuffer.handle)) }
    }
}
