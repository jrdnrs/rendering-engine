use glow::{self as gl, HasContext};

#[derive(Clone)]
pub enum FramebufferAttachment {
    Renderbuffer { internal_format: u32 },
    Texture { internal_format: u32 },
    None,
}

pub enum FramebufferAttachmentHandle {
    Renderbuffer(gl::Renderbuffer),
    Texture(gl::Texture),
    None,
}

#[derive(Clone)]
pub struct FramebufferConfig {
    pub colour: FramebufferAttachment,
    pub depth: FramebufferAttachment,
    pub stencil: FramebufferAttachment,

    pub width: i32,
    pub height: i32,
    pub samples: i32,
}

pub struct Framebuffer<'a> {
    gl: &'a gl::Context,
    pub handle: gl::Framebuffer,
    pub colour_handle: FramebufferAttachmentHandle,
    pub depth_handle: FramebufferAttachmentHandle,
    pub stencil_handle: FramebufferAttachmentHandle,

    pub config: FramebufferConfig,
}

impl<'a> Framebuffer<'a> {
    pub fn new(gl: &'a gl::Context, config: &FramebufferConfig) -> Self {
        if config.samples > 1 {
            Self::new_multisample(gl, config)
        } else {
            Self::new_one_sample(gl, config)
        }
    }

    fn new_one_sample(gl: &'a gl::Context, config: &FramebufferConfig) -> Self {
        let config = config.clone();

        let framebuffer = unsafe { gl.create_framebuffer().unwrap() };
        unsafe {
            gl.bind_framebuffer(gl::FRAMEBUFFER, Some(framebuffer));
            // gl.viewport(0, 0, config.width, config.height)
        }

        let colour_handle = match config.colour {
            FramebufferAttachment::Renderbuffer { internal_format } => {
                let colour_rbo = unsafe { gl.create_renderbuffer().unwrap() };

                unsafe {
                    gl.bind_renderbuffer(gl::RENDERBUFFER, Some(colour_rbo));
                    gl.renderbuffer_storage(
                        gl::RENDERBUFFER,
                        internal_format as u32,
                        config.width,
                        config.height,
                    );
                    gl.framebuffer_renderbuffer(
                        gl::FRAMEBUFFER,
                        gl::COLOR_ATTACHMENT0,
                        gl::RENDERBUFFER,
                        Some(colour_rbo),
                    );
                }

                FramebufferAttachmentHandle::Renderbuffer(colour_rbo)
            }
            FramebufferAttachment::Texture { internal_format } => {
                let colour_tex = unsafe { gl.create_texture().unwrap() };

                unsafe {
                    gl.bind_texture(gl::TEXTURE_2D, Some(colour_tex));
                    gl.tex_image_2d(
                        gl::TEXTURE_2D,
                        0,
                        internal_format as i32,
                        config.width,
                        config.height,
                        0,
                        gl::RGBA,
                        gl::FLOAT,
                        None,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MAG_FILTER,
                        gl::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_WRAP_S,
                        gl::CLAMP_TO_EDGE as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_WRAP_T,
                        gl::CLAMP_TO_EDGE as i32,
                    );
                    gl.framebuffer_texture_2d(
                        gl::FRAMEBUFFER,
                        gl::COLOR_ATTACHMENT0,
                        gl::TEXTURE_2D,
                        Some(colour_tex),
                        0,
                    );
                }

                FramebufferAttachmentHandle::Texture(colour_tex)
            }
            FramebufferAttachment::None => unsafe {
                gl.draw_buffer(gl::NONE);
                gl.read_buffer(gl::NONE);

                FramebufferAttachmentHandle::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachment::Renderbuffer { internal_format } => {
                let depth_rbo = unsafe { gl.create_renderbuffer().unwrap() };

                unsafe {
                    gl.bind_renderbuffer(gl::RENDERBUFFER, Some(depth_rbo));
                    gl.renderbuffer_storage(
                        gl::RENDERBUFFER,
                        internal_format as u32,
                        config.width,
                        config.height,
                    );
                    gl.framebuffer_renderbuffer(
                        gl::FRAMEBUFFER,
                        gl::DEPTH_ATTACHMENT,
                        gl::RENDERBUFFER,
                        Some(depth_rbo),
                    );
                }

                FramebufferAttachmentHandle::Renderbuffer(depth_rbo)
            }
            FramebufferAttachment::Texture { internal_format } => {
                let depth_tex = unsafe { gl.create_texture().unwrap() };

                unsafe {
                    gl.bind_texture(gl::TEXTURE_2D, Some(depth_tex));
                    gl.tex_image_2d(
                        gl::TEXTURE_2D,
                        0,
                        internal_format as i32,
                        config.width,
                        config.height,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_INT,
                        None,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MAG_FILTER,
                        gl::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_WRAP_S,
                        gl::CLAMP_TO_EDGE as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_WRAP_T,
                        gl::CLAMP_TO_EDGE as i32,
                    );
                    gl.framebuffer_texture_2d(
                        gl::FRAMEBUFFER,
                        gl::DEPTH_ATTACHMENT,
                        gl::TEXTURE_2D,
                        Some(depth_tex),
                        0,
                    );
                }

                FramebufferAttachmentHandle::Texture(depth_tex)
            }
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachment::Renderbuffer { internal_format } => {
                let stencil_rbo = unsafe { gl.create_renderbuffer().unwrap() };

                unsafe {
                    gl.bind_renderbuffer(gl::RENDERBUFFER, Some(stencil_rbo));
                    gl.renderbuffer_storage(
                        gl::RENDERBUFFER,
                        internal_format as u32,
                        config.width,
                        config.height,
                    );
                    gl.framebuffer_renderbuffer(
                        gl::FRAMEBUFFER,
                        gl::STENCIL_ATTACHMENT,
                        gl::RENDERBUFFER,
                        Some(stencil_rbo),
                    );
                }

                FramebufferAttachmentHandle::Renderbuffer(stencil_rbo)
            }
            FramebufferAttachment::Texture { internal_format } => {
                let stencil_tex = unsafe { gl.create_texture().unwrap() };

                unsafe {
                    gl.bind_texture(gl::TEXTURE_2D, Some(stencil_tex));
                    gl.tex_image_2d(
                        gl::TEXTURE_2D,
                        0,
                        internal_format as i32,
                        config.width,
                        config.height,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_INT,
                        None,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MIN_FILTER,
                        gl::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_MAG_FILTER,
                        gl::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_WRAP_S,
                        gl::CLAMP_TO_EDGE as i32,
                    );
                    gl.tex_parameter_i32(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_WRAP_T,
                        gl::CLAMP_TO_EDGE as i32,
                    );
                    gl.framebuffer_texture_2d(
                        gl::FRAMEBUFFER,
                        gl::STENCIL_ATTACHMENT,
                        gl::TEXTURE_2D,
                        Some(stencil_tex),
                        0,
                    );
                }

                FramebufferAttachmentHandle::Texture(stencil_tex)
            }
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        unsafe {
            gl.bind_framebuffer(gl::FRAMEBUFFER, None);
            gl.bind_renderbuffer(gl::RENDERBUFFER, None);
            gl.bind_texture(gl::TEXTURE_2D, None);
        }

        Self {
            gl,
            handle: framebuffer,
            colour_handle,
            depth_handle,
            stencil_handle,
            config,
        }
    }

    fn new_multisample(gl: &'a gl::Context, config: &FramebufferConfig) -> Self {
        let config = config.clone();

        let framebuffer = unsafe { gl.create_framebuffer().unwrap() };
        unsafe {
            gl.bind_framebuffer(gl::FRAMEBUFFER, Some(framebuffer));
            // gl.viewport(0, 0, config.width, config.height)
        }

        let colour_handle = match config.colour {
            FramebufferAttachment::Renderbuffer { internal_format } => {
                let colour_rbo = unsafe { gl.create_renderbuffer().unwrap() };

                unsafe {
                    gl.bind_renderbuffer(gl::RENDERBUFFER, Some(colour_rbo));
                    gl.renderbuffer_storage_multisample(
                        gl::RENDERBUFFER,
                        config.samples,
                        internal_format as u32,
                        config.width,
                        config.height,
                    );
                    gl.framebuffer_renderbuffer(
                        gl::FRAMEBUFFER,
                        gl::COLOR_ATTACHMENT0,
                        gl::RENDERBUFFER,
                        Some(colour_rbo),
                    );
                }

                FramebufferAttachmentHandle::Renderbuffer(colour_rbo)
            }
            FramebufferAttachment::Texture { internal_format } => {
                let colour_tex = unsafe { gl.create_texture().unwrap() };

                unsafe {
                    gl.bind_texture(gl::TEXTURE_2D_MULTISAMPLE, Some(colour_tex));
                    gl.tex_image_2d_multisample(
                        gl::TEXTURE_2D_MULTISAMPLE,
                        config.samples,
                        internal_format as i32,
                        config.width,
                        config.height,
                        true,
                    );
                    gl.framebuffer_texture_2d(
                        gl::FRAMEBUFFER,
                        gl::COLOR_ATTACHMENT0,
                        gl::TEXTURE_2D_MULTISAMPLE,
                        Some(colour_tex),
                        0,
                    );
                }

                FramebufferAttachmentHandle::Texture(colour_tex)
            }
            FramebufferAttachment::None => unsafe {
                gl.draw_buffer(gl::NONE);
                gl.read_buffer(gl::NONE);

                FramebufferAttachmentHandle::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachment::Renderbuffer { internal_format } => {
                let depth_rbo = unsafe { gl.create_renderbuffer().unwrap() };

                unsafe {
                    gl.bind_renderbuffer(gl::RENDERBUFFER, Some(depth_rbo));
                    gl.renderbuffer_storage_multisample(
                        gl::RENDERBUFFER,
                        config.samples,
                        internal_format as u32,
                        config.width,
                        config.height,
                    );
                    gl.framebuffer_renderbuffer(
                        gl::FRAMEBUFFER,
                        gl::DEPTH_ATTACHMENT,
                        gl::RENDERBUFFER,
                        Some(depth_rbo),
                    );
                }

                FramebufferAttachmentHandle::Renderbuffer(depth_rbo)
            }
            FramebufferAttachment::Texture { internal_format } => {
                let depth_tex = unsafe { gl.create_texture().unwrap() };

                unsafe {
                    gl.bind_texture(gl::TEXTURE_2D_MULTISAMPLE, Some(depth_tex));
                    gl.tex_image_2d_multisample(
                        gl::TEXTURE_2D_MULTISAMPLE,
                        config.samples,
                        internal_format as i32,
                        config.width,
                        config.height,
                        true,
                    );
                    gl.framebuffer_texture_2d(
                        gl::FRAMEBUFFER,
                        gl::DEPTH_ATTACHMENT,
                        gl::TEXTURE_2D_MULTISAMPLE,
                        Some(depth_tex),
                        0,
                    );
                }

                FramebufferAttachmentHandle::Texture(depth_tex)
            }
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachment::Renderbuffer { internal_format } => {
                let stencil_rbo = unsafe { gl.create_renderbuffer().unwrap() };

                unsafe {
                    gl.bind_renderbuffer(gl::RENDERBUFFER, Some(stencil_rbo));
                    gl.renderbuffer_storage_multisample(
                        gl::RENDERBUFFER,
                        config.samples,
                        internal_format as u32,
                        config.width,
                        config.height,
                    );
                    gl.framebuffer_renderbuffer(
                        gl::FRAMEBUFFER,
                        gl::STENCIL_ATTACHMENT,
                        gl::RENDERBUFFER,
                        Some(stencil_rbo),
                    );
                }

                FramebufferAttachmentHandle::Renderbuffer(stencil_rbo)
            }
            FramebufferAttachment::Texture { internal_format } => {
                let stencil_tex = unsafe { gl.create_texture().unwrap() };

                unsafe {
                    gl.bind_texture(gl::TEXTURE_2D_MULTISAMPLE, Some(stencil_tex));
                    gl.tex_image_2d_multisample(
                        gl::TEXTURE_2D_MULTISAMPLE,
                        config.samples,
                        internal_format as i32,
                        config.width,
                        config.height,
                        true,
                    );
                    gl.framebuffer_texture_2d(
                        gl::FRAMEBUFFER,
                        gl::STENCIL_ATTACHMENT,
                        gl::TEXTURE_2D_MULTISAMPLE,
                        Some(stencil_tex),
                        0,
                    );
                }

                FramebufferAttachmentHandle::Texture(stencil_tex)
            }
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        unsafe {
            gl.bind_framebuffer(gl::FRAMEBUFFER, None);
            gl.bind_renderbuffer(gl::RENDERBUFFER, None);
            gl.bind_texture(gl::TEXTURE_2D_MULTISAMPLE, None);
        }

        Self {
            gl,
            handle: framebuffer,
            colour_handle,
            depth_handle,
            stencil_handle,
            config,
        }
    }

    pub fn get_colour_texture_handle(&self) -> Option<gl::Texture> {
        if let FramebufferAttachmentHandle::Texture(handle) = self.colour_handle {
            Some(handle)
        } else {
            None
        }
    }

    pub fn get_colour_renderbuffer_handle(&self) -> Option<gl::Renderbuffer> {
        if let FramebufferAttachmentHandle::Renderbuffer(handle) = self.colour_handle {
            Some(handle)
        } else {
            None
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.config.width = width;
        self.config.height = height;
        let new = Self::new(&self.gl, &self.config);
        self.delete();
        *self = new;
    }

    pub fn bind(&self) {
        unsafe { self.gl.bind_framebuffer(gl::FRAMEBUFFER, Some(self.handle)) }
    }

    pub fn unbind(&self) {
        unsafe { self.gl.bind_framebuffer(gl::FRAMEBUFFER, None) }
    }

    pub fn delete(&self) {
        self.unbind();

        match self.colour_handle {
            FramebufferAttachmentHandle::Renderbuffer(handle) => unsafe {
                self.gl.delete_renderbuffer(handle)
            },
            FramebufferAttachmentHandle::Texture(handle) => unsafe {
                self.gl.delete_texture(handle)
            },
            FramebufferAttachmentHandle::None => (),
        }

        match self.depth_handle {
            FramebufferAttachmentHandle::Renderbuffer(handle) => unsafe {
                self.gl.delete_renderbuffer(handle)
            },
            FramebufferAttachmentHandle::Texture(handle) => unsafe {
                self.gl.delete_texture(handle)
            },
            FramebufferAttachmentHandle::None => (),
        }

        match self.stencil_handle {
            FramebufferAttachmentHandle::Renderbuffer(handle) => unsafe {
                self.gl.delete_renderbuffer(handle)
            },
            FramebufferAttachmentHandle::Texture(handle) => unsafe {
                self.gl.delete_texture(handle)
            },
            FramebufferAttachmentHandle::None => (),
        }

        unsafe { self.gl.delete_framebuffer(self.handle) }
    }
}
