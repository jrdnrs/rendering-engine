use log::error;

use crate::{
    graphics::{graphics::ApiEnum, texture::*},
    platform::rustgl as gl,
};

#[derive(Clone, Copy)]
pub enum TextureFilter {
    Nearest = gl::NEAREST as isize,
    Linear = gl::LINEAR as isize,
}

#[derive(Clone, Copy)]
pub enum TextureWrap {
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    ClampToBorder = gl::CLAMP_TO_BORDER as isize,
    Repeat = gl::REPEAT as isize,
    MirrorRepeat = gl::MIRRORED_REPEAT as isize,
    MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE as isize,
}

#[derive(Clone, Copy)]
pub enum ImageDataType {
    UnsignedByte = gl::UNSIGNED_BYTE as isize,
    UnsignedShort = gl::UNSIGNED_SHORT as isize,
    Float = gl::FLOAT as isize,
    NA,
}

#[derive(Clone, Copy)]
pub enum ImageFormat {
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
    // Bc6hTypeless = gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT as isize,
    Bc6hUnsignedFloat16 = gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT as isize,
    Bc6hSignedFloat16 = gl::COMPRESSED_RGB_BPTC_SIGNED_FLOAT as isize,
    // Bc7Typeless = gl::COMPRESSED_RGBA_BPTC_UNORM as isize,
    Bc7UnsignedNormalised = gl::COMPRESSED_RGBA_BPTC_UNORM as isize,
    Bc7UnsignedNormalisedSrgb = gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM as isize,
}

fn gl_mipmap_filter(filter: TextureFilter) -> ApiEnum {
    match filter {
        TextureFilter::Nearest => gl::NEAREST_MIPMAP_NEAREST,
        TextureFilter::Linear => gl::LINEAR_MIPMAP_LINEAR,
    }
}

fn derive_gl_internal_format(format: ApiEnum, data_type: ApiEnum, srgb: bool) -> ApiEnum {
    if srgb {
        match format {
            gl::RGB => gl::SRGB8,
            gl::RGBA => gl::SRGB8_ALPHA8,
            gl::COMPRESSED_RGBA_BPTC_UNORM => gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM,
            _ => {
                error!("Failed to find SRGB equivalent for '{}'", format);
                format
            }
        }
    } else {
        match format {
            gl::RGB => match data_type {
                gl::UNSIGNED_BYTE => gl::RGB8,
                gl::UNSIGNED_SHORT => gl::RGB16,
                gl::FLOAT => gl::RGB32F,
                _ => {
                    error!(
                        "Failed to determine internal format based on data type '{}'",
                        data_type
                    );
                    panic!();
                }
            },
            gl::RGBA => match data_type {
                gl::UNSIGNED_BYTE => gl::RGBA8,
                gl::UNSIGNED_SHORT => gl::RGBA16,
                gl::FLOAT => gl::RGBA32F,
                _ => {
                    error!(
                        "Failed to determine internal format based on data type '{}'",
                        data_type
                    );
                    panic!();
                }
            },
            _ => format,
        }
    }
}

impl Texture {
    pub fn new_2d(&self, image: Image, config: &TextureConfig) -> Texture {
        let target = gl::TEXTURE_2D;
        let config = config.clone();

        let width = image.width;
        let height = image.height;
        let data_type = image.data_type;
        let format = image.format;
        let internal_format =
            derive_gl_internal_format(format as u32, data_type as u32, config.srgb);

        let handle = unsafe {
            let handle = gl::create_named_texture(target).unwrap();
            gl::texture_storage_2d(
                handle,
                image.mipmap_count as i32,
                internal_format,
                width as i32,
                height as i32,
            );

            if image.compressed {
                // mipmaps are not automatically generated for compressed textures, should be done beforehand
                if image.mipmap_count == 0 && config.mipmap {
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

                    gl::compressed_texture_sub_image_2d(
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
                gl::texture_sub_image_2d(
                    handle,
                    0,
                    0,
                    0,
                    width as i32,
                    height as i32,
                    format as u32,
                    data_type as u32,
                    gl::PixelUnpackData::Slice(&image.bytes),
                );

                gl::generate_texture_mipmap(handle);
            }

            let min_filter = if config.mipmap {
                gl_mipmap_filter(config.min_filter)
            } else {
                config.min_filter as u32
            };

            gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_S, config.wrap as i32);
            gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_T, config.wrap as i32);
            gl::texture_parameter_i32(handle, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::texture_parameter_i32(handle, gl::TEXTURE_MAG_FILTER, config.mag_filter as i32);

            handle
        };

        Texture {
            config,
            handle: handle.0,
            target,
            internal_format,
            format: format as u32,
            data_type: data_type as u32,
            width,
            height,
        }
    }

    pub fn new_cubemap(&self, images: [Image; 6], config: &TextureConfig) -> Texture {
        let target = gl::TEXTURE_CUBE_MAP;
        let config = config.clone();

        // TODO: Ensure that all images have same format/dimensions
        let data_type = images[0].data_type;
        let format = images[0].format;
        let internal_format =
            derive_gl_internal_format(format as u32, data_type as u32, config.srgb);

        let width = images[0].width;
        let height = images[0].height;

        let handle = unsafe {
            let handle = gl::create_named_texture(target).unwrap();
            gl::texture_storage_2d(
                handle,
                images[0].mipmap_count as i32,
                internal_format,
                width as i32,
                height as i32,
            );

            // just check the first to see if they are compressed, all should be the same (still should check)
            if images[0].compressed {
                // mipmaps are not automatically generated for compressed textures, should be done beforehand
                if images[0].mipmap_count == 0 && config.mipmap {
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

                        gl::compressed_texture_sub_image_3d(
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
                    gl::texture_sub_image_3d(
                        handle,
                        0,
                        0,
                        0,
                        i as i32,
                        width as i32,
                        height as i32,
                        1,
                        format as u32,
                        data_type as u32,
                        gl::PixelUnpackData::Slice(&image.bytes),
                    );
                }

                gl::generate_texture_mipmap(handle);
            }

            let min_filter = if config.mipmap {
                gl_mipmap_filter(config.min_filter)
            } else {
                config.min_filter as u32
            };

            gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_S, config.wrap as i32);
            gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_T, config.wrap as i32);
            gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_R, config.wrap as i32);
            gl::texture_parameter_i32(handle, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::texture_parameter_i32(handle, gl::TEXTURE_MAG_FILTER, config.mag_filter as i32);

            handle
        };

        Texture {
            config,
            handle: handle.0,
            target,
            internal_format,
            format: format as u32,
            data_type: data_type as u32,
            width,
            height,
        }
    }

    pub fn bind(&self, texture: &Texture) {
        unsafe {
            gl::bind_texture(texture.target, Some(gl::GlTexture(texture.handle)));
        }
    }

    pub fn unbind(&self, texture: &Texture) {
        unsafe {
            gl::bind_texture(texture.target, None);
        }
    }

    pub fn delete(&self, texture: &Texture) {
        self.unbind(texture);
        unsafe {
            gl::delete_texture(gl::GlTexture(texture.handle));
        }
    }
}
