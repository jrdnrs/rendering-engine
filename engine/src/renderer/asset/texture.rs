#![allow(dead_code)]

use glow::{self as gl, HasContext};
use image::DynamicImage::*;
use log::error;

pub struct TextureConfig {
    pub target: u32,
    pub wrap: u32,
    pub min_filter: u32,
    pub mag_filter: u32,
    internal_format: u32,
    format: u32,
    data_type: u32,
    width: u32,
    height: u32,
}

impl TextureConfig {
    pub fn default() -> Self {
        Self {
            target: gl::TEXTURE_2D,
            wrap: gl::REPEAT,
            min_filter: gl::LINEAR_MIPMAP_LINEAR,
            mag_filter: gl::LINEAR,
            internal_format: gl::RGBA,
            format: 0,
            data_type: 0,
            width: 0,
            height: 0,
        }
    }
}

pub struct Texture<'a> {
    pub handle: gl::Texture,
    pub config: TextureConfig,
    gl: &'a gl::Context,
}

impl<'a> Texture<'a> {
    pub fn new(gl: &'a gl::Context, img: &image::DynamicImage, mut config: TextureConfig) -> Self {
        let handle = unsafe {
            let handle = gl.create_texture().unwrap();
            gl.bind_texture(config.target, Some(handle));
            gl.tex_parameter_i32(config.target, gl::TEXTURE_WRAP_S, config.wrap as i32);
            gl.tex_parameter_i32(config.target, gl::TEXTURE_WRAP_T, config.wrap as i32);
            gl.tex_parameter_i32(
                config.target,
                gl::TEXTURE_MIN_FILTER,
                config.min_filter as i32,
            );
            gl.tex_parameter_i32(
                config.target,
                gl::TEXTURE_MAG_FILTER,
                config.mag_filter as i32,
            );

            config.width = img.width();
            config.height = img.height();
            (config.format, config.data_type) = match img {
                ImageRgb8(_) => (gl::RGB, gl::UNSIGNED_BYTE),
                ImageRgba8(_) => (gl::RGBA, gl::UNSIGNED_BYTE),
                ImageRgb16(_) => (gl::RGB, gl::UNSIGNED_SHORT),
                ImageRgba16(_) => (gl::RGBA, gl::UNSIGNED_SHORT),
                ImageRgb32F(_) => (gl::RGB, gl::FLOAT),
                ImageRgba32F(_) => (gl::RGBA, gl::FLOAT),
                _ => {
                    error!("Cannot determine image format, assuming RGBA8 and continuing, please check");
                    (gl::RGBA, gl::UNSIGNED_BYTE)
                }
            };

            // TODO: match on config.target to determine which texture image to create

            gl.tex_image_2d(
                config.target,
                0,
                config.internal_format as i32,
                config.width as i32,
                config.height as i32,
                0,
                config.format,
                config.data_type,
                Some(img.as_bytes()),
            );

            gl.generate_mipmap(config.target);
            gl.bind_texture(config.target, None);

            handle
        };

        Self { handle, config, gl }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(self.config.target, Some(self.handle));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_texture(self.config.target, None);
        }
    }

    pub fn delete(&self) {
        self.unbind();
        unsafe {
            self.gl.delete_texture(self.handle);
        }
    }
}
