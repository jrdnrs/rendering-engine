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

    pub width: u32,
    pub height: u32,
    pub samples: u32,
}

impl Default for FramebufferConfig {
    fn default() -> Self {
        Self {
            colour: FramebufferAttachment::None,
            depth: FramebufferAttachment::None,
            stencil: FramebufferAttachment::None,
            width: crate::WIDTH,
            height: crate::HEIGHT,
            samples: 1,
        }
    }
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

        let framebuffer = unsafe { gl.create_named_framebuffer().unwrap() };

        let colour_handle = match config.colour {
            FramebufferAttachment::Renderbuffer { internal_format } => Self::create_renderbuffer(
                gl,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::Texture { internal_format } => Self::create_texture(
                gl,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::None => unsafe {
                gl.named_framebuffer_draw_buffer(framebuffer, gl::NONE);
                gl.named_framebuffer_read_buffer(framebuffer, gl::NONE);

                FramebufferAttachmentHandle::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachment::Renderbuffer { internal_format } => Self::create_renderbuffer(
                gl,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::Texture { internal_format } => Self::create_texture(
                gl,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachment::Renderbuffer { internal_format } => Self::create_renderbuffer(
                gl,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::Texture { internal_format } => Self::create_texture(
                gl,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

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

        let framebuffer = unsafe { gl.create_named_framebuffer().unwrap() };

        let colour_handle = match config.colour {
            FramebufferAttachment::Renderbuffer { internal_format } => Self::create_renderbuffer_multisample(
                gl,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
                config.samples
            ),
            FramebufferAttachment::Texture { internal_format } => Self::create_texture_multisample(
                gl,
                framebuffer,
                gl::COLOR_ATTACHMENT0,
                internal_format,
                config.width,
                config.height,
                config.samples
            ),
            FramebufferAttachment::None => unsafe {
                gl.named_framebuffer_draw_buffer(framebuffer, gl::NONE);
                gl.named_framebuffer_read_buffer(framebuffer, gl::NONE);

                FramebufferAttachmentHandle::None
            },
        };

        let depth_handle = match config.depth {
            FramebufferAttachment::Renderbuffer { internal_format } => Self::create_renderbuffer_multisample(
                gl,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                config.samples
            ),
            FramebufferAttachment::Texture { internal_format } => Self::create_texture_multisample(
                gl,
                framebuffer,
                gl::DEPTH_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                config.samples
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        let stencil_handle = match config.stencil {
            FramebufferAttachment::Renderbuffer { internal_format } => Self::create_renderbuffer_multisample(
                gl,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                config.samples
            ),
            FramebufferAttachment::Texture { internal_format } => Self::create_texture_multisample(
                gl,
                framebuffer,
                gl::STENCIL_ATTACHMENT,
                internal_format,
                config.width,
                config.height,
                config.samples
            ),
            FramebufferAttachment::None => FramebufferAttachmentHandle::None,
        };

        Self {
            gl,
            handle: framebuffer,
            colour_handle,
            depth_handle,
            stencil_handle,
            config,
        }
    }

    fn create_renderbuffer(
        gl: &gl::Context,
        framebuffer: gl::Framebuffer,
        attachment_type: u32,
        internal_format: u32,
        width: u32,
        height: u32,
    ) -> FramebufferAttachmentHandle {
        let rbo = unsafe { gl.create_named_renderbuffer().unwrap() };

        unsafe {
            gl.named_renderbuffer_storage(rbo, internal_format, width as i32, height as i32);
            gl.named_framebuffer_renderbuffer(framebuffer, attachment_type, gl::RENDERBUFFER, rbo);
        }

        FramebufferAttachmentHandle::Renderbuffer(rbo)
    }

    fn create_renderbuffer_multisample(
        gl: &gl::Context,
        framebuffer: gl::Framebuffer,
        attachment_type: u32,
        internal_format: u32,
        width: u32,
        height: u32,
        samples: u32,
    ) -> FramebufferAttachmentHandle {
        let rbo = unsafe { gl.create_named_renderbuffer().unwrap() };

        unsafe {
            gl.named_renderbuffer_storage_multisample(
                rbo,
                samples as i32,
                internal_format,
                width as i32,
                height as i32,
            );
            gl.named_framebuffer_renderbuffer(framebuffer, attachment_type, gl::RENDERBUFFER, rbo);
        }

        FramebufferAttachmentHandle::Renderbuffer(rbo)
    }

    fn create_texture(
        gl: &gl::Context,
        framebuffer: gl::Framebuffer,
        attachment_type: u32,
        internal_format: u32,
        width: u32,
        height: u32,
    ) -> FramebufferAttachmentHandle {
        let texture = unsafe { gl.create_named_texture(gl::TEXTURE_2D).unwrap() };

        unsafe {
            gl.texture_storage_2d(texture, 1, internal_format, width as i32, height as i32);
            gl.texture_parameter_i32(texture, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.texture_parameter_i32(texture, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.texture_parameter_i32(texture, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl.texture_parameter_i32(texture, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl.named_framebuffer_texture(framebuffer, attachment_type, texture, 0);
        }

        FramebufferAttachmentHandle::Texture(texture)
    }

    fn create_texture_multisample(
        gl: &gl::Context,
        framebuffer: gl::Framebuffer,
        attachment_type: u32,
        internal_format: u32,
        width: u32,
        height: u32,
        samples: u32,
    ) -> FramebufferAttachmentHandle {
        let texture = unsafe { gl.create_named_texture(gl::TEXTURE_2D_MULTISAMPLE).unwrap() };

        unsafe {
            gl.texture_storage_2d_multisample(
                texture,
                samples as i32,
                internal_format,
                width as i32,
                height as i32,
                true,
            );
            gl.named_framebuffer_texture(framebuffer, attachment_type, texture, 0);
        }

        FramebufferAttachmentHandle::Texture(texture)
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

    pub fn resize(&mut self, width: u32, height: u32) {
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
