use log::error;

use super::{framebuffer::InternalFormat, AccessModifier, DataType};
use crate::{
    graphics::{
        graphics::ApiEnum,
        texture::{Image, Texture, TextureConfig},
    },
    platform::rustgl as gl,
};

#[derive(Clone, Copy)]
pub enum TextureType {
    T2D = gl::TEXTURE_2D as isize,
    T2DArray = gl::TEXTURE_2D_ARRAY as isize,
    T3D = gl::TEXTURE_3D as isize,
    CubeMap = gl::TEXTURE_CUBE_MAP as isize,
}

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

fn derive_gl_internal_format(
    format: ImageFormat,
    data_type: DataType,
    srgb: bool,
) -> InternalFormat {
    if srgb {
        match format {
            ImageFormat::RGB => InternalFormat::SRGB8,
            ImageFormat::RGBA => InternalFormat::SRGB8A8,
            ImageFormat::Bc7UnsignedNormalised => InternalFormat::Bc7UnsignedNormalisedSRGB,
            _ => {
                // error!("Failed to find SRGB equivalent for '{}'", format);
                // format
                panic!();
            }
        }
    } else {
        match format {
            ImageFormat::RGB => match data_type {
                DataType::Uint8 => InternalFormat::RGB8,
                DataType::Uint16 => InternalFormat::RGB16,
                DataType::Float32 => InternalFormat::RGB32F,
                _ => {
                    // error!(
                    //     "Failed to determine internal format based on data type '{}'",
                    //     data_type
                    // );
                    panic!();
                }
            },
            ImageFormat::RGBA => match data_type {
                DataType::Uint8 => InternalFormat::RGBA8,
                DataType::Uint16 => InternalFormat::RGBA16,
                DataType::Float32 => InternalFormat::RGBA32F,
                _ => {
                    // error!(
                    //     "Failed to determine internal format based on data type '{}'",
                    //     data_type
                    // );
                    panic!();
                }
            },
            ImageFormat::Bc6hUnsignedFloat16 => InternalFormat::Bc6hUnsigned16F,
            ImageFormat::Bc6hSignedFloat16 => InternalFormat::Bc6hSigned16F,
            ImageFormat::Bc7UnsignedNormalised => InternalFormat::Bc7UnsignedNormalised,
            ImageFormat::Bc7UnsignedNormalisedSrgb => InternalFormat::Bc7UnsignedNormalisedSRGB,
        }
    }
}



impl Texture {
    pub fn new_2d(image: Image, config: &TextureConfig) -> Texture {
        let target = gl::TEXTURE_2D;
        let config = config.clone();

        let width = image.width;
        let height = image.height;
        let data_type = image.data_type;
        let format = image.format;
        let internal_format = derive_gl_internal_format(format, data_type, config.srgb);

        let handle = unsafe {
            let handle = gl::create_named_texture(target).unwrap();
            gl::texture_storage_2d(
                handle,
                image.mipmap_count as i32,
                internal_format as u32,
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
                        internal_format as u32,
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
            shader_texture_handle: None,
            resident: false,
            target,
            internal_format,
            format,
            data_type,
            width,
            height,
        }
    }

    pub fn new_cubemap(images: [Image; 6], config: &TextureConfig) -> Texture {
        let target = gl::TEXTURE_CUBE_MAP;
        let config = config.clone();

        // TODO: Ensure that all images have same format/dimensions
        let data_type = images[0].data_type;
        let format = images[0].format;
        let internal_format = derive_gl_internal_format(format, data_type, config.srgb);

        let width = images[0].width;
        let height = images[0].height;

        let handle = unsafe {
            let handle = gl::create_named_texture(target).unwrap();
            gl::texture_storage_2d(
                handle,
                images[0].mipmap_count as i32,
                internal_format as u32,
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
                            internal_format as u32,
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
            shader_texture_handle: None,
            target,
            resident: false,
            internal_format,
            format,
            data_type,
            width,
            height,
        }
    }

    pub fn new_framebuffer_texture(
        target: TextureType,
        internal_format: InternalFormat,
        layers: u32,
        levels: u32,
        samples: u32,
        width: u32,
        height: u32,
        config: &TextureConfig,
    ) -> Texture {
        let is_multisample = samples > 1;

        // TODO: take into account is_multisample, needs different target
        let handle = unsafe {
            gl::create_named_texture(target as u32).unwrap()
        };

        if is_multisample {
            unsafe {
                gl::texture_storage_2d_multisample(
                    handle,
                    samples as i32,
                    internal_format as u32,
                    width as i32,
                    height as i32,
                    true,
                );
            }
        } else {
            unsafe {
                match target {
                    TextureType::T2DArray => {
                        gl::texture_storage_3d(
                            handle,
                            levels as i32,
                            internal_format as u32,
                            width as i32,
                            height as i32,
                            layers as i32,
                        );
                    }
                    _ => {
                        gl::texture_storage_2d(
                            handle,
                            levels as i32,
                            internal_format as u32,
                            width as i32,
                            height as i32,
                        );
                    }
                }

                let min_filter = if config.mipmap {
                    gl_mipmap_filter(config.min_filter)
                } else {
                    config.min_filter as u32
                };

                gl::texture_parameter_i32(handle, gl::TEXTURE_MIN_FILTER, min_filter as i32);
                gl::texture_parameter_i32(handle, gl::TEXTURE_MAG_FILTER, config.mag_filter as i32);
                gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_S, config.wrap as i32);
                gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_T, config.wrap as i32);
                gl::texture_parameter_i32(handle, gl::TEXTURE_WRAP_R, config.wrap as i32);
                gl::texture_parameter_i32(
                    handle,
                    gl::TEXTURE_COMPARE_MODE,
                    gl::COMPARE_REF_TO_TEXTURE as i32,
                );
                gl::texture_parameter_i32(handle, gl::TEXTURE_COMPARE_FUNC, gl::GREATER as i32);
            }
        }

        Texture {
            config: config.clone(),
            handle: handle.0,
            shader_texture_handle: None,
            target: target as u32,
            resident: false,
            internal_format,
            format: ImageFormat::RGBA,
            data_type: DataType::Float32,
            width,
            height,
        }
    }

    pub fn get_shader_texture_handle(&mut self) -> u64 {
        if let Some(shader_texture_handle) = self.shader_texture_handle {
            return shader_texture_handle;
        } else {
            let shader_texture_handle =
                unsafe { gl::get_texture_handle(gl::GlTexture(self.handle)).0.get() };
            self.shader_texture_handle = Some(shader_texture_handle);
            return shader_texture_handle;
        }
    }

    pub fn make_texture_resident(&mut self) {
        if !self.resident {
            let shader_texture_handle = self.get_shader_texture_handle();
            unsafe {
                gl::make_texture_handle_resident(gl::GlTextureHandle(
                    std::num::NonZeroU64::new(shader_texture_handle).unwrap(),
                ))
            }
            self.resident = true;
        }
    }

    pub fn make_texture_non_resident(&mut self) {
        if self.resident {
            let shader_texture_handle = self.get_shader_texture_handle();
            unsafe {
                gl::make_texture_handle_non_resident(gl::GlTextureHandle(
                    std::num::NonZeroU64::new(shader_texture_handle).unwrap(),
                ))
            }
            self.resident = false;
        }
    }

    pub fn bind_image_unit(
        &self,
        unit: u32,
        level: u32,
        layer: u32,
        access: AccessModifier,
        format: InternalFormat,
    ) {
        unsafe {
            gl::bind_image_texture(
                unit,
                gl::GlTexture(self.handle),
                level as i32,
                layer > 0,
                layer as i32,
                access as u32,
                format as u32,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::bind_texture(self.target, Some(gl::GlTexture(self.handle)));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::bind_texture(self.target, None);
        }
    }

    pub fn delete(&self) {
        self.unbind();
        unsafe {
            gl::delete_texture(gl::GlTexture(self.handle));
        }
    }
}
