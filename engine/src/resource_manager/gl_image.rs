use std::{
    fs,
    io::{self, BufReader, ErrorKind, Read},
};

use glow::{self as gl, HasContext};
use image::{self, DynamicImage::*};
use log::error;

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 0)
        | ((array[1] as u32) << 8)
        | ((array[2] as u32) << 16)
        | ((array[3] as u32) << 24)
}

fn read_u32_le<R: Read>(r: &mut BufReader<R>) -> Result<u32, io::Error> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(as_u32_le(&buf))
}

pub enum DxgiFormat {
    Bc7Typeless = 97,
    Bc7UnsignedNormalised = 98,
    Bc7UnsignedNormalisedSrgb = 99,
}

#[derive(Debug)]
pub struct Header {
    pub flags: u32,
    pub height: u32,
    pub width: u32,
    pub pitch_or_linear_size: u32,
    pub depth: u32,
    pub mipmap_count: u32,
    pub pixel_format: PixelFormat,
    pub caps: u32,
    pub caps2: u32,
}

impl Header {
    pub fn from_reader<R: Read>(r: &mut BufReader<R>) -> Result<Self, io::Error> {
        let size = read_u32_le(r)?;
        let flags = read_u32_le(r)?;
        let height = read_u32_le(r)?;
        let width = read_u32_le(r)?;
        let pitch_or_linear_size = read_u32_le(r)?;
        let depth = read_u32_le(r)?;
        let mipmap_count = read_u32_le(r)?;
        // Skip `dwReserved1`
        {
            let mut skipped = [0; 4 * 11];
            r.read_exact(&mut skipped)?;
        }
        let pixel_format = PixelFormat::from_reader(r)?;
        let caps = read_u32_le(r)?;
        let caps2 = read_u32_le(r)?;
        // Skip `dwCaps3`, `dwCaps4`, `dwReserved2` (unused)
        {
            let mut skipped = [0; 4 + 4 + 4];
            r.read_exact(&mut skipped)?;
        }

        Ok(Self {
            flags,
            height,
            width,
            pitch_or_linear_size,
            depth,
            mipmap_count,
            pixel_format,
            caps,
            caps2,
        })
    }
}

#[derive(Debug)]
pub struct DX10Header {
    pub dxgi_format: u32,
    pub resource_dimension: u32,
    pub misc_flag: u32,
    pub array_size: u32,
    pub misc_flags_2: u32,
}

impl DX10Header {
    pub fn from_reader<R: Read>(r: &mut BufReader<R>) -> Result<Self, io::Error> {
        let dxgi_format = read_u32_le(r)?;
        let resource_dimension = read_u32_le(r)?;
        let misc_flag = read_u32_le(r)?;
        let array_size = read_u32_le(r)?;
        let misc_flags_2 = read_u32_le(r)?;

        let dx10_header = Self {
            dxgi_format,
            resource_dimension,
            misc_flag,
            array_size,
            misc_flags_2,
        };
        Ok(dx10_header)
    }
}

#[derive(Debug)]
pub struct PixelFormat {
    pub flags: u32,
    pub fourcc: [u8; 4],
    pub rgb_bit_count: u32,
    pub r_bit_mask: u32,
    pub g_bit_mask: u32,
    pub b_bit_mask: u32,
    pub a_bit_mask: u32,
}

impl PixelFormat {
    pub fn from_reader<R: Read>(r: &mut BufReader<R>) -> Result<Self, io::Error> {
        let size = read_u32_le(r)?;

        Ok(Self {
            flags: read_u32_le(r)?,
            fourcc: {
                let mut v = [0; 4];
                r.read_exact(&mut v)?;
                v
            },
            rgb_bit_count: read_u32_le(r)?,
            r_bit_mask: read_u32_le(r)?,
            g_bit_mask: read_u32_le(r)?,
            b_bit_mask: read_u32_le(r)?,
            a_bit_mask: read_u32_le(r)?,
        })
    }
}

#[derive(Debug)]
pub struct BptcImage {
    pub header: Header,
    pub dx10_header: DX10Header,
    pub bytes: Vec<u8>,
}

impl BptcImage {
    pub fn from_path(path: &'static str) -> Result<Self, io::Error> {
        let file = fs::File::open(path).unwrap();
        let mut reader = io::BufReader::new(file);

        let mut magic = [0; 4];
        reader.read_exact(&mut magic)?;

        if magic != "DDS ".as_bytes() {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let header = Header::from_reader(&mut reader)?;
        let dx10_header = DX10Header::from_reader(&mut reader)?;

        if !(dx10_header.dxgi_format >= 94 && dx10_header.dxgi_format <= 99) {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let mut bytes = Vec::new();
        let bytes_read = reader.read_to_end(&mut bytes);

        Ok(Self {
            header,
            dx10_header,
            bytes,
        })
    }
}



#[derive(Default)]
pub struct GlImage {
    pub path: &'static str,
    pub format: u32,
    pub internal_format: u32,
    pub compressed: bool,
    pub mipmap_count: u32,
    pub data_type: u32,
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

impl GlImage {
    pub fn from_path(path: &'static str) -> Result<Self, io::Error> {
        // check if it's bc7 format which Image crate doesn't support
        if let Ok(bptc_image) = BptcImage::from_path(path) {
            let format = match bptc_image.dx10_header.dxgi_format {
                94 /* DXGI_FORMAT_BC6H_TYPELESS  */=> {gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT}, // I have no idea what this should be
                95 /* DXGI_FORMAT_BC6H_UF16  */=> {gl::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT}, 
                96 /* DXGI_FORMAT_BC6H_SF16  */=> {gl::COMPRESSED_RGB_BPTC_SIGNED_FLOAT},
                97 /* DXGI_FORMAT_BC7_TYPELESS */=> {gl::COMPRESSED_RGBA_BPTC_UNORM}, // I have no idea what this should be
                98 /* DXGI_FORMAT_BC7_UNORM  */=> {gl::COMPRESSED_RGBA_BPTC_UNORM},
                99 /* DXGI_FORMAT_BC7_UNORM_SRGB  */=> {gl::COMPRESSED_SRGB_ALPHA_BPTC_UNORM},
                _ => return Err(io::Error::from(ErrorKind::InvalidData))
            };

            return Ok(Self {
                path,
                format,
                internal_format: format,
                compressed: true,
                mipmap_count: bptc_image.header.mipmap_count,
                data_type: 0, // unused
                width: bptc_image.header.width,
                height: bptc_image.header.height,
                bytes: bptc_image.bytes,
            });
        }

        if let Ok(image) = image::open(path) {
            let (format, internal_format, data_type) = match image {
                ImageRgb8(_) => (gl::RGB, gl::RGB8, gl::UNSIGNED_BYTE),
                ImageRgba8(_) => (gl::RGBA, gl::RGBA8,gl::UNSIGNED_BYTE),
                ImageRgb16(_) => (gl::RGB, gl::RGB16, gl::UNSIGNED_SHORT),
                ImageRgba16(_) => (gl::RGBA, gl::RGBA16, gl::UNSIGNED_SHORT),
                ImageRgb32F(_) => (gl::RGB, gl::RGB32F, gl::FLOAT),
                ImageRgba32F(_) => (gl::RGBA, gl::RGBA32F, gl::FLOAT),
                _ => {
                    return Err(io::Error::from(ErrorKind::InvalidData));
                }
            };
            return Ok(Self {
                path,
                format,
                internal_format,
                compressed: false,
                mipmap_count: 1,
                data_type,
                width: image.width(),
                height: image.height(),
                bytes: image.as_bytes().to_vec(),
            });
        }

        return Err(io::Error::from(ErrorKind::InvalidData));
    }
}
