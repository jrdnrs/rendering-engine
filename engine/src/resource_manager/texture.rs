#![allow(dead_code)]

use glow::{self as gl, HasContext};
use log::error;

use super::gl_image::GlImage;

#[derive(Clone)]
pub struct TextureConfig {
    pub wrap: u32,
    pub min_filter: u32,
    pub mag_filter: u32,
    pub srgb: bool,
}

fn get_internal_format(format: u32, srgb: bool) -> u32 {
    if srgb {
        match format {
            gl::RGB8 | gl::RGB16 | gl::RGB32F => gl::SRGB8,
            gl::RGBA8 | gl::RGBA16 | gl::RGBA32F => gl::SRGB8_ALPHA8,
            gl::COMPRESSED_RGBA_BPTC_UNORM => gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
            _ => {
                error!("Failed to find SRGB equivalent for '{}'", format);
                format
            }
        }
    } else {
        format
    }
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            wrap: gl::REPEAT,
            min_filter: gl::LINEAR,
            mag_filter: gl::NEAREST,
            srgb: false,
        }
    }
}

pub struct Texture<'a> {
    pub handle: gl::Texture,
    pub config: TextureConfig,
    gl: &'a gl::Context,
    target: u32,
    internal_format: u32,
    format: u32,
    data_type: u32,
    width: u32,
    height: u32,
}

impl<'a> Texture<'a> {
    pub fn new_2d(gl: &'a gl::Context, image: GlImage, config: &TextureConfig) -> Self {
        let target = gl::TEXTURE_2D;
        let config = config.clone();
        let width = image.width;
        let height = image.height;
        let format = image.format;
        let internal_format = get_internal_format(image.internal_format, config.srgb);
        let data_type = image.data_type;

        let handle = unsafe {
            let handle = gl.create_named_texture(target).unwrap();
            gl.texture_storage_2d(
                handle,
                image.mipmap_count as i32,
                internal_format,
                width as i32,
                height as i32,
            );

            if image.compressed {
                // mipmaps are not automatically generated for compressed textures, should be done beforehand
                if image.mipmap_count == 0
                    && ((config.mag_filter >= 9984 && config.mag_filter <= 9987)
                        || (config.min_filter >= 9984 && config.min_filter <= 9987))
                {
                    error!(
                        "'{}' has no mipmaps but the texture config specifies mipmap usage for filtering",
                        image.path
                    )
                }

                // one bptc block is 4x4 pixels (16 bytes/128 bits per block)
                let mut level_w = width;
                let mut level_h = height;
                let mut level_blocks: u32;
                let mut level_size: u32;
                let mut level_offset = 0;

                for level in 0..image.mipmap_count {
                    level_blocks =
                        ((level_w as f32 / 4.0).ceil() * (level_h as f32 / 4.0).ceil()) as u32;
                    level_size = level_blocks * 16;

                    gl.compressed_texture_sub_image_2d(
                        handle,
                        level as i32,
                        0,
                        0,
                        level_w as i32,
                        level_h as i32,
                        internal_format,
                        gl::CompressedPixelUnpackData::Slice(
                            &image.bytes
                                [level_offset as usize..(level_offset + level_size) as usize],
                        ),
                    );

                    level_w /= 2;
                    level_h /= 2;
                    level_offset += level_size;
                }
            } else {
                gl.texture_sub_image_2d(
                    handle,
                    0,
                    0,
                    0,
                    width as i32,
                    height as i32,
                    format,
                    data_type,
                    gl::PixelUnpackData::Slice(&image.bytes),
                );

                gl.generate_texture_mipmap(handle);
            }

            gl.texture_parameter_i32(handle, gl::TEXTURE_WRAP_S, config.wrap as i32);
            gl.texture_parameter_i32(handle, gl::TEXTURE_WRAP_T, config.wrap as i32);
            gl.texture_parameter_i32(handle, gl::TEXTURE_MIN_FILTER, config.min_filter as i32);
            gl.texture_parameter_i32(handle, gl::TEXTURE_MAG_FILTER, config.mag_filter as i32);

            handle
        };

        Self {
            handle,
            config,
            gl,
            target,
            internal_format,
            format,
            data_type,
            width,
            height,
        }
    }

    pub fn new_cubemap(gl: &'a gl::Context, images: [GlImage; 6], config: &TextureConfig) -> Self {
        let target = gl::TEXTURE_CUBE_MAP;
        let config = config.clone();

        // TODO: Ensure that all images have same format/dimensions
        let format = images[0].format;
        let internal_format = get_internal_format(images[0].internal_format, config.srgb);
        let width = images[0].width;
        let height = images[0].height;
        let data_type = images[0].data_type;

        let handle = unsafe {
            let handle = gl.create_named_texture(target).unwrap();
            gl.texture_storage_2d(
                handle,
                images[0].mipmap_count as i32,
                internal_format,
                width as i32,
                height as i32,
            );

            // just check the first to see if they are compressed, all should be the same (still should check)
            if images[0].compressed {
                // mipmaps are not automatically generated for compressed textures, should be done beforehand
                if images[0].mipmap_count == 0
                    && (config.mag_filter >= 9984 && config.mag_filter <= 9987)
                    || (config.min_filter >= 9984 && config.min_filter <= 9987)
                {
                    error!(
                        "'{}' and probably other skybox texture have no mipmaps but the texture config specifies 
                        mipmap usage for filtering",
                        images[0].path
                    )
                }

                for (i, image) in images.iter().enumerate() {
                    // bptc block is 4x4 (16 bytes/128 bits per block)
                    let mut level_w = width;
                    let mut level_h = height;
                    let mut level_blocks: u32;
                    let mut level_size: u32;
                    let mut level_offset = 0;

                    for level in 0..image.mipmap_count {
                        level_blocks =
                            ((level_w as f32 / 4.0).ceil() * (level_h as f32 / 4.0).ceil()) as u32;
                        level_size = level_blocks * 16;

                        gl.compressed_texture_sub_image_3d(
                            handle,
                            level as i32,
                            0,
                            0,
                            i as i32,
                            level_w as i32,
                            level_h as i32,
                            1,
                            internal_format,
                            gl::CompressedPixelUnpackData::Slice(
                                &image.bytes
                                    [level_offset as usize..(level_offset + level_size) as usize],
                            ),
                        );

                        level_w /= 2;
                        level_h /= 2;
                        level_offset += level_size;
                    }
                }
            } else {
                for (i, image) in images.iter().enumerate() {
                    gl.texture_sub_image_3d(
                        handle,
                        0,
                        0,
                        0,
                        i as i32,
                        width as i32,
                        height as i32,
                        1,
                        format,
                        data_type,
                        gl::PixelUnpackData::Slice(&image.bytes),
                    );
                }

                gl.generate_texture_mipmap(handle);
            }

            gl.texture_parameter_i32(handle, gl::TEXTURE_WRAP_S, config.wrap as i32);
            gl.texture_parameter_i32(handle, gl::TEXTURE_WRAP_T, config.wrap as i32);
            gl.texture_parameter_i32(handle, gl::TEXTURE_WRAP_R, config.wrap as i32);
            gl.texture_parameter_i32(handle, gl::TEXTURE_MIN_FILTER, config.min_filter as i32);
            gl.texture_parameter_i32(handle, gl::TEXTURE_MAG_FILTER, config.mag_filter as i32);

            handle
        };

        Self {
            handle,
            config,
            gl,
            data_type,
            target,
            internal_format,
            format,
            width,
            height,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(self.target, Some(self.handle));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_texture(self.target, None);
        }
    }

    pub fn delete(&self) {
        self.unbind();
        unsafe {
            self.gl.delete_texture(self.handle);
        }
    }
}
